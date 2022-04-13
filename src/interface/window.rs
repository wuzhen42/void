use pollster;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu;
use wgpu::util::DeviceExt;

use super::layout::*;
use super::{context::*, DrawBuffer};
use crate::prim::*;

pub struct Window {
    layout: Layout,
    cursor: Pnt2,
    size: Vector2<u32>,
}

impl Window {
    pub fn new(layout: Layout) -> Window {
        Window {
            layout,
            cursor: Pnt2::zero(),
            size: Vector2::<u32>::zero(),
        }
    }

    pub fn run(mut self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut context = pollster::block_on(ContextGlobal::init_wgpu(&window));
        self.size = Vector2::<u32>::new(context.config.width, context.config.height);
        self.layout.for_each(|x| x.init(&context));

        let shader = context
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Pure Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("pure.wgsl").into()),
            });

        let render_pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pure Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Pure Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[VertexColor::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[wgpu::ColorTargetState {
                        format: context.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        event_loop.run(move |event, _, contrl_flow| {
            *contrl_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !self.on_window_event(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *contrl_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                self.resize(&mut context, *physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.resize(&mut context, **new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    self.render(&context, &pipeline);
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    fn render(&mut self, context_global: &ContextGlobal, pipeline: &wgpu::RenderPipeline) {
        let output = context_global.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let buffer: DrawBuffer = self
            .layout
            .filter_map(|panel| panel.draw())
            .into_iter()
            .sum();

        let mut encoder =
            context_global
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Outliner Render Encoder"),
                });

        let vertex_buffer =
            context_global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Vertex Buffer"),
                    contents: bytemuck::cast_slice(&buffer.vertices_cd.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            context_global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Index Buffer"),
                    contents: bytemuck::cast_slice(&buffer.indices_cd.as_slice()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Outliner Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        renderpass.set_pipeline(pipeline);
        renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..buffer.indices_cd.len() as u32, 0, 0..1);
        drop(renderpass);

        let context = Context {
            global: &context_global,
            frame: ContextFrame { view },
        };
        let command_buffers = self
            .layout
            .filter_map(|panel| panel.render(&context))
            .into_iter()
            .chain(std::iter::once(encoder.finish()));

        context_global.queue.submit(command_buffers);
        output.present();
    }

    fn resize(&mut self, context: &mut ContextGlobal, size: winit::dpi::PhysicalSize<u32>) {
        // update context
        context.config.width = size.width;
        context.config.height = size.height;
        context.surface.configure(&context.device, &context.config);

        // update window
        self.size = Vector2::<u32>::new(size.width, size.height);
        self.layout.for_each(|x| x.on_window_stretch(self.size));
    }

    fn on_window_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => *keycode == VirtualKeyCode::Escape,
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = Pnt2::new(
                    position.x as f64 / self.size.x as f64,
                    position.y as f64 / self.size.y as f64,
                );
                true
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                self.on_mouse_up(*button, self.cursor);
                true
            }
            _ => false,
        }
    }

    fn on_mouse_up(&mut self, _button: MouseButton, _position: Pnt2) {}
}

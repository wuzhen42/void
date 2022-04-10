use pollster;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu;
use wgpu::util::DeviceExt;

use super::context::Context;
use super::layout::Layout;
use super::{DrawBuffer, Panel};
use crate::prim::{Pnt2, Rect};

pub struct Window {
    layout: Layout,
    panels: Vec<Box<dyn Panel>>,
}

impl Window {
    pub fn new() -> Window {
        Window {
            layout: Layout::new(Rect::from_corner(
                Pnt2::new(-1.0, -1.0),
                Pnt2::new(1.0, 1.0),
            )),
            panels: vec![],
        }
    }

    pub fn append(&mut self, panel: Box<dyn Panel>) {
        self.panels.push(panel);
        self.layout.grow();
    }

    pub fn size(&self) -> usize {
        self.panels.len()
    }

    pub fn run(self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let context = pollster::block_on(Context::init_wgpu(&window));

        event_loop.run(move |event, _, contrl_flow| {
            *contrl_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *contrl_flow = ControlFlow::Exit,
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    self.render(&context);
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    pub fn render(&self, context: &Context) {
        let output = context.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let drawbuffer = self
            .panels
            .iter()
            .zip(self.layout.leaves())
            .map(|(panel, rect)| panel.draw(rect))
            .fold(DrawBuffer::new(), |acc, x| acc.chain(x));

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("UI Render Encoder"),
            });

        let vertex_buffer_pure = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UI Vertex Buffer"),
                contents: bytemuck::cast_slice(&drawbuffer.vertices_pure.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer_pure = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UI Index Buffer"),
                contents: bytemuck::cast_slice(&drawbuffer.indices_pure.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });
        let vertex_buffer_uv = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Texture Vertex Buffer"),
                contents: bytemuck::cast_slice(&drawbuffer.vertices_uv.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer_uv = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Texture Index Buffer"),
                contents: bytemuck::cast_slice(&drawbuffer.indices_uv.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        renderpass.set_pipeline(&context.pipeline_pure);
        renderpass.set_vertex_buffer(0, vertex_buffer_pure.slice(..));
        renderpass.set_index_buffer(index_buffer_pure.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..drawbuffer.indices_pure.len() as u32, 0, 0..1);

        renderpass.set_pipeline(&context.pipeline_texture);
        renderpass.set_bind_group(0, &context.diffuse_bind_group, &[]);
        renderpass.set_vertex_buffer(0, vertex_buffer_uv.slice(..));
        renderpass.set_index_buffer(index_buffer_uv.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..drawbuffer.indices_uv.len() as u32, 0, 0..1);
        drop(renderpass);

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

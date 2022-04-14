use wgpu::util::DeviceExt;
use winit::event::VirtualKeyCode;

use super::camera;
use crate::interface::*;
use crate::prim::*;

struct RenderBackend {
    pub pipeline: wgpu::RenderPipeline,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
}

#[derive(Default)]
pub struct Viewport {
    rect: Rect,
    window_ratio: f64,
    camera: camera::Camera,
    backend: Option<RenderBackend>,
}

impl Viewport {
    fn update_camera_aspect(&mut self) {
        let rect_ratio = self.rect.width() as f64 / self.rect.height() as f64;
        self.camera.aspect = self.window_ratio * rect_ratio;
    }
}

impl Panel for Viewport {
    fn init(&mut self, context: &ContextGlobal) {
        let device = &context.device;
        let config = &context.config;

        // init camera
        self.window_ratio = config.width as f64 / config.height as f64;
        self.camera = camera::Camera {
            eye: Pnt3::new(0.0, 1.0, 2.0),
            target: Pnt3::zero(),
            up: Vec3::axisy(),
            aspect: 1.0,
            fovy: 45.0f64.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        };
        self.update_camera_aspect();
        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&self.camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // init diffuse bind group
        let diffuse_bytes = include_bytes!("happy-tree.png");
        let diffuse_texture =
            Texture::from_bytes(&device, &context.queue, diffuse_bytes, "happy-tree").unwrap();
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("texture.wgsl").into()),
        });

        // init pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Viewport Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Viewport Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexUV::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
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

        self.backend = Some(RenderBackend {
            pipeline,
            diffuse_bind_group,
            camera_buffer,
            camera_bind_group,
        })
    }

    fn render(&self, context: &Context) -> Option<wgpu::CommandBuffer> {
        let backend = self.backend.as_ref().unwrap();

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&self.camera);
        context.global.queue.write_buffer(
            &backend.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        let mut encoder =
            context
                .global
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Viewport Render Encoder"),
                });

        let mut buffer = DrawBuffer::default();
        buffer.rect_uv(self.rect);
        let vertex_buffer =
            context
                .global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Vertex Buffer"),
                    contents: bytemuck::cast_slice(&buffer.vertices_uv.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            context
                .global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Index Buffer"),
                    contents: bytemuck::cast_slice(&buffer.indices_uv.as_slice()),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Outliner Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &context.frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        renderpass.set_pipeline(&backend.pipeline);
        renderpass.set_bind_group(0, &backend.diffuse_bind_group, &[]);
        renderpass.set_bind_group(1, &backend.camera_bind_group, &[]);
        renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..buffer.indices_uv.len() as u32, 0, 0..1);
        drop(renderpass);
        Some(encoder.finish())
    }
}

impl Widget for Viewport {
    fn on_window_stretch(&mut self, size: Vector2<u32>) {
        self.window_ratio = size.x as f64 / size.y as f64;
        self.update_camera_aspect();
    }

    fn resize(&mut self, rect: Rect) {
        self.rect = rect;
        self.update_camera_aspect();
    }

    fn onclick(&mut self, cursor: Pnt2) -> bool {
        if self.rect.contains(cursor) {
            true
        } else {
            false
        }
    }

    fn onkeydown(&mut self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.camera.eye.z -= 1.0;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.camera.eye.z += 1.0;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.camera.eye.x -= 1.0;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.camera.eye.x += 1.0;
                true
            }
            _ => false,
        }
    }
}

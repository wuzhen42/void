use wgpu::util::DeviceExt;

use crate::interface::*;
use crate::prim::Rect;

#[derive(Default)]
pub struct Viewport {
    pipeline: Option<wgpu::RenderPipeline>,
    diffuse_bind_group: Option<wgpu::BindGroup>,
}

impl Panel for Viewport {
    fn init(&mut self, context: &ContextGlobal) {
        let device = &context.device;
        let config = &context.config;

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
        self.diffuse_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        }));

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("texture.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Viewport Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
        self.pipeline = Some(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            }),
        );
    }

    fn render(&self, rect: Rect, context: &Context) -> wgpu::CommandBuffer {
        let mut encoder =
            context
                .global
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Viewport Render Encoder"),
                });

        let mut buffer = DrawBuffer::new();
        buffer.rect_uv(rect);
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
                view: &context.local.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        renderpass.set_pipeline(self.pipeline.as_ref().unwrap());
        renderpass.set_bind_group(0, self.diffuse_bind_group.as_ref().unwrap(), &[]);
        renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..buffer.indices_uv.len() as u32, 0, 0..1);
        drop(renderpass);
        encoder.finish()
    }
}

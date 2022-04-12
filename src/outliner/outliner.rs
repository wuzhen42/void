use wgpu::util::DeviceExt;

use crate::interface::*;
use crate::prim::{Rect, RGB};

#[derive(Default)]
pub struct Outliner {
    pipeline: Option<wgpu::RenderPipeline>,
}

impl Panel for Outliner {
    fn init(&mut self, context: &ContextGlobal) {
        let device = &context.device;
        let config = &context.config;

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Pure Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("pure.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pure Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        self.pipeline = Some(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        let mut buffer = DrawBuffer::new();
        buffer.rect(rect, RGB::BLUE);

        let mut encoder =
            context
                .global
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Outliner Render Encoder"),
                });

        let vertex_buffer_pure =
            context
                .global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Vertex Buffer"),
                    contents: bytemuck::cast_slice(&buffer.vertices_pure.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer_pure =
            context
                .global
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Outliner Index Buffer"),
                    contents: bytemuck::cast_slice(&buffer.indices_pure.as_slice()),
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
        renderpass.set_vertex_buffer(0, vertex_buffer_pure.slice(..));
        renderpass.set_index_buffer(index_buffer_pure.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..buffer.indices_pure.len() as u32, 0, 0..1);
        drop(renderpass);
        encoder.finish()
    }
}

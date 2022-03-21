use super::draw;
use super::{context::Context, draw::DrawBuffer};
use wgpu;
use wgpu::util::DeviceExt;

use crate::prim::{bound::Rect, color::RGB, point::Pnt2};

pub struct Panel {
    rect: Rect,
    children: Vec<Panel>,
}

impl Panel {
    pub fn new() -> Panel {
        Panel {
            rect: crate::prim::bound::Rect::new(),
            children: vec![],
        }
    }

    pub fn add(&mut self, child: Panel) {
        self.children.push(child);
    }

    pub fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let output = context.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut buffer = DrawBuffer::new();
            draw::rect(
                Pnt2 { x: -1f32, y: -1f32 },
                Pnt2 { x: 1f32, y: 1f32 },
                RGB::red(),
                &mut buffer,
            );

            let vertex_buffer =
                context
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&buffer.vertices.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            let index_buffer =
                context
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&buffer.indices.as_slice()),
                        usage: wgpu::BufferUsages::INDEX,
                    });

            let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
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
            renderpass.set_pipeline(&context.pipeline);
            renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            renderpass.draw_indexed(0..buffer.indices.len() as u32, 0, 0..1);
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

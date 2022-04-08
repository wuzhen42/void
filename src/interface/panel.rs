use super::{context::Context, draw::DrawBuffer};
use wgpu;
use wgpu::util::DeviceExt;

use crate::prim::{bound::Rect, color::RGB, point::Pnt2, vector::Vec2};

#[derive(Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Panel {
    rect: Rect,
    children: Vec<Panel>,
    orient: Orientation,
    background: RGB,
    margin: f64,
}

impl Panel {
    pub fn new() -> Panel {
        Self::bg(RGB::red())
    }

    pub fn bg(color: RGB) -> Panel {
        Panel {
            rect: Rect {
                min: Pnt2 { x: -1.0, y: -1.0 },
                max: Pnt2 { x: 1.0, y: 1.0 },
            },
            children: vec![],
            orient: Orientation::Horizontal,
            background: color,
            margin: 0.02,
        }
    }

    pub fn add(&mut self, child: Panel) {
        self.children.push(child);

        let offset = match self.orient {
            Orientation::Horizontal => Vec2 {
                x: (self.rect.max.x - self.rect.min.x),
                y: 0.0,
            },
            Orientation::Vertical => Vec2 {
                x: 0.0,
                y: (self.rect.max.y - self.rect.min.y),
            },
        } / self.children.len() as f64;
        let block_margin = match self.orient {
            Orientation::Horizontal => Vec2 {
                x: self.margin / 2.0,
                y: self.margin,
            },
            Orientation::Vertical => Vec2 {
                x: self.margin,
                y: self.margin / 2.0,
            },
        };
        let num_children = self.children.len();
        // reaverage size for all children
        for (i, child) in self.children.iter_mut().enumerate() {
            child.rect = Rect {
                min: self.rect.min + offset * i as f64 + block_margin,
                max: self.rect.max - offset * (num_children - i - 1) as f64 - block_margin,
            };
        }
        let border_extra_margin = match self.orient {
            Orientation::Horizontal => Vec2 {
                x: self.margin / 2.0,
                y: 0.0,
            },
            Orientation::Vertical => Vec2 {
                x: 0.0,
                y: self.margin / 2.0,
            },
        };
        self.children.first_mut().unwrap().rect.min += border_extra_margin;
        self.children.last_mut().unwrap().rect.max -= border_extra_margin;
    }

    pub fn draw(&self, buffer: &mut DrawBuffer) {
        buffer.rect(self.rect, self.background);
        for child in self.children.iter() {
            child.draw(buffer);
        }
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

        let mut buffer = DrawBuffer::new();
        self.draw(&mut buffer);

        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&buffer.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = context
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
        drop(renderpass);

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_rect_after_add() {
        let mut panel = Panel::new();
        assert_eq!(panel.orient, Orientation::Horizontal);
        assert_eq!(panel.margin, 0.02);

        panel.add(Panel::new());
        let rect0 = panel.children[0].rect;
        assert_eq!(rect0.min, Pnt2 { x: -0.98, y: -0.98 });
        assert_eq!(rect0.max, Pnt2 { x: 0.98, y: 0.98 });

        panel.add(Panel::new());
        let rect0 = panel.children[0].rect;
        assert_eq!(rect0.min, Pnt2 { x: -0.98, y: -0.98 });
        assert_eq!(rect0.max, Pnt2 { x: -0.01, y: 0.98 });
        let rect1 = panel.children[1].rect;
        assert_eq!(rect1.min, Pnt2 { x: 0.01, y: -0.98 });
        assert_eq!(rect1.max, Pnt2 { x: 0.98, y: 0.98 });
    }
}

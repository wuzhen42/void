use super::context::{VertexColor, VertexUV};
use crate::prim::{Rect, RGB};

pub struct DrawBuffer {
    pub vertices_pure: Vec<VertexColor>,
    pub indices_pure: Vec<u16>,
    pub vertices_uv: Vec<VertexUV>,
    pub indices_uv: Vec<u16>,
}

pub trait Panel {
    fn draw(&self, rect: Rect) -> DrawBuffer;
}

impl DrawBuffer {
    pub fn new() -> DrawBuffer {
        DrawBuffer {
            vertices_pure: vec![],
            indices_pure: vec![],
            vertices_uv: vec![],
            indices_uv: vec![],
        }
    }

    pub fn chain(self, other: DrawBuffer) -> DrawBuffer {
        let offset = self.vertices_pure.len() as u16;
        let vertices_pure = [
            self.vertices_pure.as_slice(),
            &other.vertices_pure.as_slice(),
        ]
        .concat();
        let indices_pure = self
            .indices_pure
            .into_iter()
            .chain(other.indices_pure.iter().map(|x| x + offset))
            .collect();

        let offset = self.vertices_uv.len() as u16;
        let vertices_uv = [self.vertices_uv.as_slice(), &other.vertices_uv.as_slice()].concat();
        let indices_uv = self
            .indices_uv
            .into_iter()
            .chain(other.indices_uv.iter().map(|x| x + offset))
            .collect();

        DrawBuffer {
            vertices_pure,
            indices_pure,
            vertices_uv,
            indices_uv,
        }
    }

    pub fn rect(&mut self, rect: Rect, color: RGB) {
        let color = [color.r as f32, color.g as f32, color.b as f32];
        let base = self.vertices_pure.len() as u16;
        self.vertices_pure.push(VertexColor {
            position: [rect.min.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices_pure.push(VertexColor {
            position: [rect.max.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices_pure.push(VertexColor {
            position: [rect.min.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.vertices_pure.push(VertexColor {
            position: [rect.max.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.indices_pure.extend(vec![
            base + 0,
            base + 1,
            base + 2,
            base + 2,
            base + 1,
            base + 3,
        ]);
    }

    pub fn rect_uv(&mut self, rect: Rect) {
        let base = self.vertices_uv.len() as u16;
        self.vertices_uv.push(VertexUV {
            position: [rect.min.x as f32, rect.min.y as f32, 0.0],
            uv: [0.0, 1.0],
        });
        self.vertices_uv.push(VertexUV {
            position: [rect.max.x as f32, rect.min.y as f32, 0.0],
            uv: [1.0, 1.0],
        });
        self.vertices_uv.push(VertexUV {
            position: [rect.min.x as f32, rect.max.y as f32, 0.0],
            uv: [0.0, 0.0],
        });
        self.vertices_uv.push(VertexUV {
            position: [rect.max.x as f32, rect.max.y as f32, 0.0],
            uv: [1.0, 0.0],
        });
        self.indices_uv.extend(vec![
            base + 0,
            base + 1,
            base + 2,
            base + 2,
            base + 1,
            base + 3,
        ]);
    }
}

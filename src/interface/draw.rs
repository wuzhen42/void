use super::context::Vertex;
use crate::prim::{Rect, RGB};

pub struct DrawBuffer {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub trait Panel {
    fn draw(&self, rect: Rect) -> DrawBuffer;
}

impl DrawBuffer {
    pub fn new() -> DrawBuffer {
        DrawBuffer {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn chain(self, other: DrawBuffer) -> DrawBuffer {
        let offset = self.vertices.len() as u16;
        let vertices = [self.vertices.as_slice(), &other.vertices.as_slice()].concat();
        let indices = self
            .indices
            .into_iter()
            .chain(other.indices.iter().map(|x| x + offset))
            .collect();
        DrawBuffer { vertices, indices }
    }

    pub fn rect(&mut self, rect: Rect, color: RGB) {
        let color = [color.r as f32, color.g as f32, color.b as f32];
        let base = self.vertices.len() as u16;
        self.vertices.push(Vertex {
            position: [rect.min.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices.push(Vertex {
            position: [rect.max.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices.push(Vertex {
            position: [rect.min.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.vertices.push(Vertex {
            position: [rect.max.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.indices.extend(vec![
            base + 0,
            base + 1,
            base + 2,
            base + 2,
            base + 1,
            base + 3,
        ]);
    }
}

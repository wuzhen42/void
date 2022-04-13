use super::context::{VertexColor, VertexUV};
use crate::prim::{Rect, RGB};

#[derive(Default)]
pub struct DrawBuffer {
    pub vertices_cd: Vec<VertexColor>,
    pub indices_cd: Vec<u16>,
    pub vertices_uv: Vec<VertexUV>,
    pub indices_uv: Vec<u16>,
}

impl DrawBuffer {
    pub fn chain(self, other: DrawBuffer) -> DrawBuffer {
        let offset = self.vertices_cd.len() as u16;
        let vertices_cd = [self.vertices_cd.as_slice(), &other.vertices_cd.as_slice()].concat();
        let indices_cd = self
            .indices_cd
            .into_iter()
            .chain(other.indices_cd.iter().map(|x| x + offset))
            .collect();

        let offset = self.vertices_uv.len() as u16;
        let vertices_uv = [self.vertices_uv.as_slice(), &other.vertices_uv.as_slice()].concat();
        let indices_uv = self
            .indices_uv
            .into_iter()
            .chain(other.indices_uv.iter().map(|x| x + offset))
            .collect();

        DrawBuffer {
            vertices_cd,
            indices_cd,
            vertices_uv,
            indices_uv,
        }
    }

    pub fn rect(&mut self, rect: Rect, color: RGB) {
        let color = [color.r as f32, color.g as f32, color.b as f32];
        let base = self.vertices_cd.len() as u16;
        self.vertices_cd.push(VertexColor {
            position: [rect.min.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices_cd.push(VertexColor {
            position: [rect.max.x as f32, rect.min.y as f32, 0.0],
            color,
        });
        self.vertices_cd.push(VertexColor {
            position: [rect.min.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.vertices_cd.push(VertexColor {
            position: [rect.max.x as f32, rect.max.y as f32, 0.0],
            color,
        });
        self.indices_cd.extend(vec![
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

impl std::iter::Sum for DrawBuffer {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |a, b| a.chain(b))
    }
}

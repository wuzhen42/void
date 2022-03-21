use super::context::Vertex;
use crate::prim::{color::RGB, point::Pnt2};

pub struct DrawBuffer {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl DrawBuffer {
    pub fn new() -> DrawBuffer {
        DrawBuffer {
            vertices: vec![],
            indices: vec![],
        }
    }
}

pub fn rect(min: Pnt2, max: Pnt2, color: RGB, buffer: &mut DrawBuffer) {
    let color = [color.r, color.g, color.b];
    let base = buffer.vertices.len() as u16;
    buffer.vertices.push(Vertex {
        position: [min.x, min.y, 0.0],
        color,
    });
    buffer.vertices.push(Vertex {
        position: [max.x, min.y, 0.0],
        color,
    });
    buffer.vertices.push(Vertex {
        position: [min.x, max.y, 0.0],
        color,
    });
    buffer.vertices.push(Vertex {
        position: [max.x, max.y, 0.0],
        color,
    });
    let indices = vec![base + 0, base + 1, base + 2, base + 2, base + 1, base + 3];
    buffer.indices.extend(indices);
}

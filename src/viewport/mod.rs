use crate::interface::{DrawBuffer, Panel};
use crate::prim::{Pnt3, Rect, Vec3, RGB};

pub struct Viewport {
    camera: Camera,
}

impl Viewport {
    fn new(size: [usize; 2]) -> Viewport {
        let camera = Camera {
            eye: Pnt3::new(0.0, 1.0, 2.0),
            target: Pnt3::ZERO,
            up: Vec3::Y,
            aspect: size[0] as f64 / size[1] as f64,
            fovy: 45.0,
            znear: 1.0,
            zfar: 100.0,
        };
        Viewport { camera }
    }
}

impl Panel for Viewport {
    fn draw(&self, rect: Rect) -> DrawBuffer {
        let mut buffer = DrawBuffer::new();
        buffer.rect(rect, RGB::BLUE);
        return buffer;
    }
}

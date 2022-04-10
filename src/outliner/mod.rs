use crate::interface::{DrawBuffer, Panel};
use crate::prim::{Rect};

pub struct Outliner {}

impl Panel for Outliner {
    fn draw(&self, rect: Rect) -> DrawBuffer {
        let mut buffer = DrawBuffer::new();
        buffer.rect_uv(rect);
        return buffer;
    }
}

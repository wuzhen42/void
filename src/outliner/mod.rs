use crate::interface::{DrawBuffer, Panel};
use crate::prim::{Rect, RGB};

pub struct Outliner {}

impl Panel for Outliner {
    fn draw(&self, rect: Rect) -> DrawBuffer {
        let mut buffer = DrawBuffer::new();
        buffer.rect(rect, RGB::green());
        return buffer;
    }
}

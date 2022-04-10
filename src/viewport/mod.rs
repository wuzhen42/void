use crate::interface::{DrawBuffer, Panel};
use crate::prim::{Rect, RGB};

pub struct Viewport {}

impl Panel for Viewport {
    fn draw(&self, rect: Rect) -> DrawBuffer {
        let mut buffer = DrawBuffer::new();
        buffer.rect(rect, RGB::BLUE);
        return buffer;
    }
}

use crate::interface::*;
use crate::prim::{Rect, RGB};

#[derive(Default)]
pub struct Outliner {
    rect: Rect,
}

impl Panel for Outliner {
    fn resize(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&self) -> Option<DrawBuffer> {
        let mut draw = DrawBuffer::default();
        draw.rect(self.rect, RGB::BLUE);
        Some(draw)
    }
}

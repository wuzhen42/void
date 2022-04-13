use super::context::*;
use super::draw::DrawBuffer;
use crate::prim::*;

pub trait Panel {
    fn init(&mut self, _: &ContextGlobal) {}

    fn draw(&self) -> Option<DrawBuffer> {
        None
    }

    fn render(&self, _: &Context) -> Option<wgpu::CommandBuffer> {
        None
    }

    fn on_window_stretch(&mut self, _: Vector2<u32>) {}

    fn resize(&mut self, rect: Rect);
}

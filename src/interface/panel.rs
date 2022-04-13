use super::context::*;
use super::draw::DrawBuffer;
use crate::prim::*;

pub trait Panel {
    #[allow(unused_variables)]
    fn init(&mut self, context: &ContextGlobal) {}

    fn draw(&self) -> Option<DrawBuffer> {
        None
    }

    #[allow(unused_variables)]
    fn render(&self, context: &Context) -> Option<wgpu::CommandBuffer> {
        None
    }

    fn on_window_stretch(&mut self, _size: Vector2<u32>) {}

    fn resize(&mut self, rect: Rect);

    fn onclick(&mut self, cursor: Pnt2) -> bool;
}

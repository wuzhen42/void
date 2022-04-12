use crate::prim::Rect;
use super::context::*;

pub trait Panel {
    fn init(&mut self, context: &ContextGlobal);

    fn render(&self, rect: Rect, context: &Context) -> wgpu::CommandBuffer;
}

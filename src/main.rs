#![allow(dead_code)]

mod interface;
mod outliner;
mod prim;
mod viewport;

use interface::*;
use prim::*;

fn main() {
    let mut layout = Layout::new(
        Rect::from_corner(Pnt2::new(-1.0, -1.0), Pnt2::new(1.0, 1.0)),
        Orientation::Vertical,
    );
    layout.grow(Box::new(outliner::Outliner::default()));
    layout.grow(Box::new(viewport::Viewport::default()));
    Window::new(layout).run();
}

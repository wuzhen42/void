mod interface;
mod outliner;
mod prim;
mod viewport;

use interface::Window;

fn main() {
    let mut window = Window::new();
    window.append(Box::new(outliner::Outliner::default()));
    window.append(Box::new(viewport::Viewport::default()));
    window.run();
}

mod interface;
mod prim;

use interface::window::Window;

fn main() {
    let window = Window::new();
    window.run();
}

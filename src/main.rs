mod interface;
mod outliner;
mod prim;
mod viewport;

use interface::Window;

fn main() {
    let mut window = Window::new();
    window.append(Box::new(outliner::Outliner {}));
    window.append(Box::new(viewport::Viewport {}));
    assert_eq!(window.size(), 2);

    window.run();
}

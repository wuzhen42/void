#[derive(Clone, Copy)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            width: 1.0,
            height: 1.0,
        }
    }
}

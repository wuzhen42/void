use super::{point::Pnt2, vector::Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub min: Pnt2,
    pub max: Pnt2,
}

impl Rect {
    pub fn center(&self) -> Pnt2 {
        (self.min + self.max) / 2.0
    }

    pub fn extent(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn new(center: Pnt2, extent: Vec2) -> Rect {
        Rect {
            min: center - extent / 2.0,
            max: center + extent / 2.0,
        }
    }
}

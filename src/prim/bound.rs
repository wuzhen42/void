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

    pub fn topleft(&self) -> Pnt2 {
        Pnt2 {
            x: self.min.x,
            y: self.max.y,
        }
    }

    pub fn bottomleft(&self) -> Pnt2 {
        self.min
    }

    pub fn topright(&self) -> Pnt2 {
        self.max
    }

    pub fn empty() -> Rect {
        Rect {
            min: Pnt2::ZERO,
            max: Pnt2::ZERO,
        }
    }

    pub fn from_corner(bottomleft: Pnt2, topright: Pnt2) -> Rect {
        Rect {
            min: bottomleft,
            max: topright,
        }
    }

    pub fn from_center(center: Pnt2, extent: Vec2) -> Rect {
        Rect {
            min: center - extent / 2.0,
            max: center + extent / 2.0,
        }
    }
}

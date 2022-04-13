use super::{point::Pnt2, vector::Vec2};

#[derive(Clone, Copy, Debug, Default)]
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

    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn bottomleft(&self) -> Pnt2 {
        self.min
    }

    pub fn bottomright(&self) -> Pnt2 {
        Pnt2::new(self.max.x, self.min.y)
    }

    pub fn topleft(&self) -> Pnt2 {
        Pnt2::new(self.min.x, self.max.y)
    }

    pub fn topright(&self) -> Pnt2 {
        self.max
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
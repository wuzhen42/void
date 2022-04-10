use super::vector::Vec2;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pnt2 {
    pub x: f64,
    pub y: f64,
}

impl Pnt2 {
    pub fn new(x: f64, y: f64) -> Pnt2 {
        Pnt2 { x, y }
    }

    pub fn zero() -> Pnt2 {
        Pnt2 { x: 0.0, y: 0.0 }
    }
}

impl ops::Add<Pnt2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, rhs: Pnt2) -> Self::Output {
        Pnt2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Pnt2> for Pnt2 {
    type Output = Vec2;
    fn sub(self, rhs: Pnt2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div<f64> for Pnt2 {
    type Output = Pnt2;
    fn div(self, rhs: f64) -> Self::Output {
        Pnt2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Add<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Pnt2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2> for Pnt2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Pnt2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vec2> for Pnt2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

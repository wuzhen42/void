use super::{Vec2, Vec3};
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pnt2 {
    pub x: f64,
    pub y: f64,
}

impl Pnt2 {
    pub const fn new(x: f64, y: f64) -> Pnt2 {
        Pnt2 { x, y }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0);
}

impl ops::Add<Pnt2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, rhs: Pnt2) -> Self::Output {
        Pnt2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Pnt2> for Pnt2 {
    type Output = Vec2;
    fn sub(self, rhs: Pnt2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Div<f64> for Pnt2 {
    type Output = Pnt2;
    fn div(self, rhs: f64) -> Self::Output {
        Pnt2::new(self.x / rhs, self.y / rhs)
    }
}

impl ops::Add<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Pnt2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Vec2> for Pnt2 {
    type Output = Pnt2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Pnt2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::AddAssign<Vec2> for Pnt2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<Vec2> for Pnt2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pnt3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Pnt3 {
        Pnt3 { x, y, z }
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Add<Pnt3> for Pnt3 {
    type Output = Pnt3;
    fn add(self, rhs: Pnt3) -> Self::Output {
        Pnt3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Pnt3> for Pnt3 {
    type Output = Vec3;
    fn sub(self, rhs: Pnt3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

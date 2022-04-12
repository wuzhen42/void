use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f64) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z * rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn from_slice(e: &[f64]) -> Vec4 {
        Vec4::new(e[0], e[1], e[2], e[3])
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Index<usize> for Vec4 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &f64::NAN,
        }
    }
}

impl std::iter::Sum for Vec4 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec4>,
    {
        iter.fold(Vec4::ZERO, |a, b| a + b)
    }
}

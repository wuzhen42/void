use num::traits::*;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
pub type Vec2 = Vector2<f64>;

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Zero> Vector2<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Sub<Output = T>> Sub<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<Output = T>> Mul<Vector2<T>> for Vector2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub type Vec3 = Vector3<f64>;

impl<T> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Zero + One> Vector3<T> {
    pub fn axisx() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    pub fn axisy() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    pub fn axisz() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: Float> Vector3<T> {
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl<T: Sub<Output = T>> Sub<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
pub type Vec4 = Vector4<f64>;

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Copy> Vector4<T> {
    pub fn from_slice(e: &[T]) -> Self {
        Self::new(e[0], e[1], e[2], e[3])
    }
}

impl<T: Zero> Vector4<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

impl<T: Add<Output = T>> Add<Vector4<T>> for Vector4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl<T> Index<usize> for Vector4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index {} out of range [0, 3]", index),
        }
    }
}

impl<T: Zero> std::iter::Sum for Vector4<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized() {
        assert_eq!(
            Vec3::new(0.0, 1.0, 2.0).normalize(),
            Vec3::new(0.0, 0.4472135954999579277, 0.89442719099991585541)
        );
    }
}

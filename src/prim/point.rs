use super::vector::*;
use num::traits::*;
use std::ops::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}
pub type Pnt2 = Point2<f64>;

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl<T: Zero> Point2<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Add<Output = T>> Add<Point2<T>> for Point2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub<Point2<T>> for Point2<T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Point2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Add<Output = T>> Add<Vector2<T>> for Point2<T> {
    type Output = Self;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Self;
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: std::ops::AddAssign> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: std::ops::SubAssign> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
pub type Pnt3 = Point3<f64>;

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Zero> Point3<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Add<Output = T> + Mul<Output = T>> Point3<T> {
    pub fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Add<Output = T>> Add<Vector3<T>> for Point3<T> {
    type Output = Self;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

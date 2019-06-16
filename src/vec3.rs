use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Add + AddAssign + Sub + SubAssign,
{
    fn from(item: (T, T, T)) -> Self {
        Vec3 {
            x: item.0,
            y: item.1,
            z: item.2,
        }
    }
}

impl<T> Vec3<T>
where
    T: Add + AddAssign + Sub + SubAssign,
{
    pub fn new(p: (T, T, T)) -> Self {
        Vec3 {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<T, Output = T> + AddAssign,
{
    type Output = Self;
    fn add(self, other: Self) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Add + Sub<Output = T> + AddAssign + SubAssign,
{
    type Output = Self;
    fn sub(self, other: Self) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

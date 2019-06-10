use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Vec2<T: Add + AddAssign> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Vec2<T>
where
    T: Add + AddAssign + Sub + SubAssign,
{
    fn from(item: (T, T)) -> Self {
        Vec2 {
            x: item.0,
            y: item.1,
        }
    }
}

impl<T> Vec2<T>
where
    T: Add + AddAssign + Sub + SubAssign,
{
    pub fn new(p: (T, T)) -> Self {
        Vec2 { x: p.0, y: p.1 }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<T, Output = T> + AddAssign,
{
    type Output = Self;
    fn add(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Add + Sub<Output = T> + AddAssign + SubAssign,
{
    type Output = Self;
    fn sub(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

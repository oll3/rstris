#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl <T> Vec2 <T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {x: x, y: y}
    }
}

pub trait ToVec2<T> {
    fn to_vec2(&self) -> Vec2<T>;
}

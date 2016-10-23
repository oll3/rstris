use vec2::*;
use std::ops::Add;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl ToVec2<i32> for Position {
    fn to_vec2(&self) -> Vec2<i32> {
        Vec2::new(self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position{x: x, y: y}
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn add(&mut self, offs_x: i32, offs_y: i32) {
        self.x += offs_x;
        self.y += offs_y;
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x,
                      self.y + other.y)
    }
}

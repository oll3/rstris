use crate::movement::*;
use crate::position::*;
use crate::vec3::*;

pub type PosDir = Vec3<i32>;

impl PosDir {
    pub fn apply_move(&self, movement: &Movement) -> Self {
        let mut pos = self.clone();
        match *movement {
            Movement::MoveLeft => pos.x -= 1,
            Movement::MoveRight => pos.x += 1,
            Movement::MoveDown => pos.y += 1,
            Movement::MoveUp => pos.y -= 1,
            Movement::RotateCW => pos.z += 1,
            Movement::RotateCCW => pos.z -= 1,
        };
        pos
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_dir(&self) -> i32 {
        self.z
    }
    pub fn get_pos(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
    pub fn normalize_dir(&mut self, num_directions: usize) {
        if self.z < 0 {
            // Handle negative rotation
            self.z = num_directions as i32 + self.z;
        }
        self.z %= num_directions as i32;
    }
}

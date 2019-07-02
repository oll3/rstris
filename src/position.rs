use crate::movement::Movement;
use crate::vec2::Vec2;
use crate::vec3::Vec3;

pub type Position = Vec3<i32>;

impl Position {
    pub fn apply_move(&self, movement: Movement) -> Self {
        let mut pos = *self;
        match movement {
            Movement::MoveLeft => pos.x -= 1,
            Movement::MoveRight => pos.x += 1,
            Movement::MoveDown => pos.y += 1,
            Movement::MoveUp => pos.y -= 1,
            Movement::RotateCW => pos.z += 1,
            Movement::RotateCCW => pos.z -= 1,
        };
        pos
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn dir(&self) -> i32 {
        self.z
    }
    pub fn xy(&self) -> Vec2<i32> {
        Vec2::new((self.x, self.y))
    }
    pub fn normalize_dir(&mut self, num_directions: u8) {
        let num_directions = i32::from(num_directions);
        while self.z < 0 {
            // Handle negative rotation
            self.z += num_directions;
        }
        self.z %= num_directions;
    }
}

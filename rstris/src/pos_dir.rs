use vec3::*;
use position::*;
use movement::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PosDir {
    pos: Position,
    dir: i32,
}

impl ToVec3<i32> for PosDir {
    fn to_vec3(&self) -> Vec3<i32> {
        Vec3::new(self.pos.get_x(), self.pos.get_y(), self.dir)
    }
}

impl PosDir {
    pub fn new(x: i32, y: i32, dir: i32) -> Self {
        PosDir{pos: Position::new(x, y), dir: dir}
    }
    pub fn apply_move(pos1: &PosDir, movement: &Movement) -> Self {
        let mut pos = pos1.clone();
        match *movement {
            Movement::MoveLeft => {pos.pos.add(-1, 0)},
            Movement::MoveRight => {pos.pos.add(1, 0)},
            Movement::MoveDown => {pos.pos.add(0, 1)},
            Movement::MoveUp => {pos.pos.add(0, -1)},
            Movement::RotateCW => {pos.dir += 1},
            Movement::RotateCCW => {pos.dir -= 1},
        };
        pos
    }
    pub fn get_x(&self) -> i32 {
        self.pos.get_x()
    }
    pub fn get_y(&self) -> i32 {
        self.pos.get_y()
    }
    pub fn get_dir(&self) -> i32 {
        self.dir
    }
    pub fn get_pos(&self) -> &Position {
        &self.pos
    }
    pub fn normalize_dir(&mut self, num_directions: usize) {
        if self.dir < 0 {
            // Handle negative rotation
            self.dir = num_directions as i32 + self.dir;
        }
        self.dir %= num_directions as i32;
    }
}

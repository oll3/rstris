#[derive(Clone, Debug)]
pub struct Position {
    dir: i32,
    x_pos: i32,
    y_pos: i32,
}

#[allow(dead_code)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    RotateCW,
    RotateCCW,
}

impl Position {
    pub fn new(x: i32, y: i32, dirf: i32) -> Position {
        Position{x_pos: x, y_pos: y, dir: dirf}
    }
    pub fn apply_move(pos1: &Position, movement: &Movement) -> Position {
        let mut pos = pos1.clone();
        match *movement {
            Movement::MoveLeft => {pos.x_pos -= 1},
            Movement::MoveRight => {pos.x_pos += 1},
            Movement::MoveDown => {pos.y_pos += 1},
            Movement::MoveUp => {pos.y_pos -= 1},
            Movement::RotateCW => {pos.dir += 1},
            Movement::RotateCCW => {pos.dir -= 1},
        };
        pos
    }
    pub fn get_x(&self) -> i32 {
        self.x_pos
    }
    pub fn get_y(&self) -> i32 {
        self.y_pos
    }
    pub fn get_dir(&self) -> i32 {
        self.dir
    }
    pub fn normalize_dir(&mut self, num_directions: usize) {
        if self.dir < 0 {
            // Handle negative rotation
            self.dir = num_directions as i32 + self.dir;
        }
        self.dir %= num_directions as i32;
    }
}

use std::ops::Add;
use std::cmp::Ordering;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PosDir {
    pos: Position,
    dir: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    RotateCW,
    RotateCCW,
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
}

impl Add for Position {
    type Output = Position;
    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x,
                      self.y + other.y)
    }
}

impl PosDir {
    pub fn new(x: i32, y: i32, dir: i32) -> Self {
        PosDir{pos: Position::new(x, y), dir: dir}
    }
    pub fn apply_move(pos1: &PosDir, movement: &Movement) -> Self {
        let mut pos = pos1.clone();
        match *movement {
            Movement::MoveLeft => {pos.pos.x -= 1},
            Movement::MoveRight => {pos.pos.x += 1},
            Movement::MoveDown => {pos.pos.y += 1},
            Movement::MoveUp => {pos.pos.y -= 1},
            Movement::RotateCW => {pos.dir += 1},
            Movement::RotateCCW => {pos.dir -= 1},
        };
        pos
    }
    pub fn get_x(&self) -> i32 {
        self.pos.x
    }
    pub fn get_y(&self) -> i32 {
        self.pos.y
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

#[derive(Debug)]
pub struct MoveAndTime {
    pub movement: Movement,
    pub time: u64,
}
impl Ord for MoveAndTime {
    fn cmp(&self, other: &MoveAndTime) -> Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for MoveAndTime {
    fn partial_cmp(&self, other: &MoveAndTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for MoveAndTime {}
impl PartialEq for MoveAndTime {
    fn eq(&self, other: &MoveAndTime) -> bool { self.time == other.time }
}

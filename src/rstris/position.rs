#[derive(Clone, Debug)]
pub struct Position {
    dir: i32,
    x_pos: i32,
    y_pos: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, dirf: i32) -> Position {
        Position{x_pos: x, y_pos: y, dir: dirf}
    }
    pub fn add_pos(pos1: &Position, pos2: &Position) -> Position {
        Position{x_pos: pos1.x_pos + pos2.x_pos,
                 y_pos: pos1.y_pos + pos2.y_pos,
                 dir: pos1.dir + pos2.dir}
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

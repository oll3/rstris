extern crate rand;

#[derive(Clone, Debug)]
pub struct FigureDir {
    pub blocks: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct Figure {
    figure_name: String,
    pub dir: Vec<FigureDir>,
}

#[derive(Clone, Debug)]
pub struct Position {
    dir: i32,
    x_pos: i32,
    y_pos: i32,
}

#[derive(Debug)]
pub struct Player<'a> {
    player_name: String,
    current_pos: Position,
    avail_figures: &'a Vec<Figure>,
    next_figure: Figure,
    current_figure: Option<Figure>,
}

#[derive(Debug)]
pub struct Playfield {
    pf_name: String,
    pf_width: usize,
    pf_height: usize,
    blocks: Vec<Vec<u8>>,
}

impl Position {
    pub fn new(x: i32, y: i32, dirf: i32) -> Position {
        Position{x_pos: x, y_pos: y, dir: dirf}
    }

    fn add_pos(pos1: &Position, pos2: &Position) -> Position {
        Position{x_pos: pos1.x_pos + pos2.x_pos,
                 y_pos: pos1.y_pos + pos2.y_pos,
                 dir: pos1.dir + pos2.dir}
    }
}

impl Figure {
    pub fn new(name: String) -> Figure {
        Figure{figure_name: name, dir: vec![]}
    }
    pub fn add_direction(&mut self, dir_blocks: Vec<Vec<u8>>) {
        self.dir.push(FigureDir{blocks: dir_blocks});
    }
    fn get_fig_dir(&self, dir_index: usize) -> FigureDir {
        let dir_index = dir_index % self.dir.len();
        return self.dir[dir_index].clone();
    }

    fn place_figure(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.dir as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.y_pos + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                let pos_x = pos.x_pos + col as i32;
                if b != 0 && pf.contains(pos_x, pos_y) {
                    pf.blocks[pos_y as usize][pos_x as usize] = b;
                }
            }
        }
    }

    fn remove_figure(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.dir as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.y_pos + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                let pos_x = pos.x_pos + col as i32;
                if b != 0 && pf.contains(pos_x, pos_y) {
                    pf.blocks[pos_y as usize][pos_x as usize] = 0;
                }
            }
        }
    }

    fn test_figure(&self, pf: &Playfield, pos: &Position) -> bool {
        let fig_dir = self.get_fig_dir(pos.dir as usize);
        for row in 0..fig_dir.blocks.len() {
            let offs_y = pos.y_pos + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let offs_x = pos.x_pos + col as i32;
                let b = fig_dir.blocks[row][col];
                if b != 0 {
                    if !pf.contains(offs_x, offs_y) {
                        return false;
                    } else if pf.blocks[offs_y as usize][offs_x as usize] != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn move_figure(&self, pf: &mut Playfield,
                   current_pos: &Position,
                   new_pos: &Position) -> bool {
        self.remove_figure(pf, current_pos);
        if self.test_figure(pf, new_pos) {
            self.place_figure(pf, new_pos);
            return true;
        }
        self.place_figure(pf, current_pos);
        return false;
    }
}


impl <'a> Player<'a> {
    pub fn new(name: String, figures: &Vec<Figure>) -> Player {
        Player{player_name: name,
               avail_figures: figures,
               current_pos: Position::new(-1, -1, -1),
               current_figure: None,
               next_figure: Player::get_next_figure(figures),
        }
    }

    //
    // Get a random figure from array of figures
    //
    fn get_next_figure(avail_figures: &Vec<Figure>) -> Figure {
        let next_figure = (rand::random::<u8>() %
                           avail_figures.len() as u8) as usize;
        let figure = avail_figures[next_figure].clone();
        return figure;
    }

    //
    // Generate next figure to be placed
    //
    pub fn gen_next_figure(&mut self) {
        self.next_figure = Player::get_next_figure(self.avail_figures);
        println!("{}: Next figure is {}", self.player_name,
                 self.next_figure.figure_name);
    }

    //
    // Place the next figure in playfield.
    // Game is over if this function returns false.
    //
    pub fn place_next_figure(&mut self, pf: &mut Playfield) -> bool {
        self.current_pos.dir = 0;
        self.current_pos.x_pos = (pf.width() / 2 - 1) as i32;
        self.current_pos.y_pos = 0;
        self.current_figure = Some(self.next_figure.clone());

        let figure = self.current_figure.clone().unwrap();
        if !figure.test_figure(pf, &self.current_pos) {
            return false;
        } else {
            self.next_figure.place_figure(pf, &self.current_pos);
            println!("{}: Placed figure {} in playfield",
                     self.player_name, self.next_figure.figure_name);

            self.gen_next_figure();
        }
        return true;
    }

    //
    // Move figure to new (relative) position.
    // If the move is downwards and fails false is returned, else true.
    //
    pub fn move_figure(&mut self, pf: &mut Playfield,
                       rel_pos: &Position) -> bool {

        let figure = self.current_figure.clone().unwrap();
        let mut new_pos = Position::add_pos(&self.current_pos, &rel_pos);
        let n_dirs = figure.dir.len() as i32;
        if new_pos.dir < 0 {
            // Handle negative rotation
            new_pos.dir = n_dirs + new_pos.dir;
        }
        new_pos.dir %= n_dirs;
        let result = figure.move_figure(pf, &self.current_pos, &new_pos);
        if result {
            self.current_pos = new_pos;
            return true;
        } else {
            return rel_pos.y_pos == 0;
        }
    }
}


impl Playfield {
    pub fn new(name:String, width: usize, height: usize) -> Playfield {
        let mut playfield = Playfield{pf_name: name,
                                      pf_width: width,
                                      pf_height: height,
                                      blocks: vec![]};
        for _ in 0..height {
            playfield.blocks.push(vec![0; width as usize]);
        }
        playfield
    }
    pub fn get_block(&self, x: usize, y: usize) -> u8 {
        self.blocks[y][x]
    }
    pub fn width(&self) -> usize {
        self.pf_width
    }
    pub fn height(&self) -> usize {
        self.pf_height
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.pf_width as i32 &&
            y >= 0 && y < self.pf_height as i32
    }
    fn block_is_set(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) != 0
    }

    //
    // Search playfield for full lines (returned in order of top to bottom)
    //
    pub fn find_full_lines(&self) -> Vec<usize> {
        let mut full_lines: Vec<usize> = vec![];

        for y in 0..self.pf_height {
            let mut line_full = true;
            for x in 0..self.pf_width {
                if !self.block_is_set(x, y) {
                    line_full = false;
                    break;
                }
            }
            if line_full {
                full_lines.push(y);
            }
        }
        return full_lines;
    }

    //
    // Remove a line from playfield and move all lines above downwards
    //
    pub fn throw_line(&mut self, line: usize) {
        let mut y = line as i32;
        while y >= 0 {
            for x in 0..self.pf_width {
                if y >= 1 {
                    self.blocks[y as usize][x] = self.blocks[y as usize - 1][x];
                } else {
                    self.blocks[y as usize][x] = 0;
                }
            }
            y -= 1;
        }
    }
}

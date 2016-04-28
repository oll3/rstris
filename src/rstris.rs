extern crate rand;

#[derive(Clone, Debug)]
pub struct FigureDir {
    pub blocks: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct Figure {
    pub dir: Vec<FigureDir>,
}

#[derive(Clone, Debug)]
struct FigurePosition {
    fig_index: i32,
    fig_dir: i32,
    fig_x_pos: i32,
    fig_y_pos: i32,
}


#[derive(Debug)]
pub struct Playfield {
    pf_width: usize,
    pf_height: usize,
    blocks: Vec<Vec<u8>>,
    figures: Vec<Figure>,
    figure: FigurePosition,
}

#[derive(Debug)]
pub struct RSTris {
    playfields: Vec<Playfield>,
    /*
    current_figure: i32,
    fig_dir: i32,
    fig_x_pos: i32,
    fig_y_pos: i32,
*/
}


impl Playfield {
    pub fn new(width: usize, height: usize) -> Playfield {
        let mut playfield = Playfield{pf_width: width,
                                      pf_height: height,
                                      blocks: vec![],
                                      figures: vec![],
                                      figure:
                                      FigurePosition{fig_index: -1,
                                                     fig_dir: -1,
                                                     fig_x_pos: 0,
                                                     fig_y_pos: 0}};
        for _ in 0..height {
            playfield.blocks.push(vec![0; width as usize]);
        }
        playfield
    }
    pub fn add_figure(&mut self, fig: Figure) {
        self.figures.push(fig);
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

    fn place_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {
        let fig_dir = fig.dir[dir as usize].clone();
        for row in 0..fig_dir.blocks.len() {
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                if b != 0 {
                    self.blocks[y as usize + row][x as usize + col] = b;
                }
            }
        }
    }

    fn rm_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32) {
        let fig_dir = fig.dir[dir as usize].clone();
        for row in 0..fig_dir.blocks.len() {
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                if b != 0 {
                    self.blocks[y as usize + row][x as usize + col] = 0;
                }
            }
        }
    }

    fn test_figure(&self, fig: &Figure, dir: i32, x: i32, y: i32) -> bool {
        let fig_dir = fig.dir[dir as usize].clone();
        for row in 0..fig_dir.blocks.len() {
            let offs_y = y + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let offs_x = x + col as i32;
                let b = fig_dir.blocks[row][col];
                if b != 0 {
                    if offs_y < 0 || offs_y >= self.pf_height as i32 {
                        return false;
                    }
                    if offs_x < 0 || offs_x >= self.pf_width as i32 {
                        return false;
                    }
                    if self.blocks[offs_y as usize][offs_x as usize] != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn move_figure(&mut self, fig: &Figure, dir: i32, x: i32, y: i32,
                   x_offs: i32, y_offs: i32) -> bool {
        self.rm_figure(fig, dir, x, y);
        if self.test_figure(fig, dir, x + x_offs, y + y_offs) {
            self.place_figure(fig, dir, x + x_offs, y + y_offs);
            return true;
        }
        self.place_figure(fig, dir, x, y);
        return false;
    }

    fn update(&mut self) {
        let mut figure = self.figure.clone();
        if figure.fig_index == -1 {
            figure.fig_index =
                (rand::random::<u8>() % self.figures.len() as u8) as i32;
            figure.fig_dir = 0;
            figure.fig_x_pos = (self.width() / 2 - 1) as i32;
            figure.fig_y_pos = 0;
            let fig = self.figures[figure.fig_index as usize].clone();
            if self.test_figure(&fig, figure.fig_dir,
                                figure.fig_x_pos,
                                figure.fig_y_pos) {
                self.place_figure(&fig, figure.fig_dir,
                                  figure.fig_x_pos,
                                  figure.fig_y_pos);
            }
            else {
                // Game over?
                figure.fig_index = -2;
                println!("Game Over!");
            }
        }
        else if figure.fig_index >= 0 {
            let fig = self.figures[figure.fig_index as usize].clone();
            if self.move_figure(&fig,
                                figure.fig_dir,
                                figure.fig_x_pos,
                                figure.fig_y_pos,
                                0, 1) {
                figure.fig_y_pos += 1;
            }
            else {
                // Figure couldn't be moved further - Leave where ut is and
                // place another figure.
                figure.fig_index = -1;
            }
        }
        self.figure = figure;
    }
}

impl RSTris {

    pub fn new() -> RSTris {
        RSTris{playfields: vec![]}
    }

    pub fn add_playfield(&mut self, playfield: Playfield) {
        self.playfields.push(playfield);
    }

    pub fn get_playfields(&self) -> &Vec<Playfield> {
        &self.playfields
    }

    pub fn update(&mut self) {
        for i in 0..self.playfields.len() {
            self.playfields.get_mut(i).unwrap().update();
        }
    }
}

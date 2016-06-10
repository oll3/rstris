use rstris::playfield::*;
use rstris::position::*;

#[derive(Clone, Debug)]
pub struct FigureDir {
    width: usize,
    height: usize,
    blocks: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct Figure {
    figure_name: String,
    pub dir: Vec<FigureDir>,
}

impl FigureDir {
    pub fn new(dir_blocks: Vec<Vec<u8>>) -> FigureDir {
        FigureDir{height: dir_blocks.len(),
                  width: dir_blocks[0].len(),
                  blocks: dir_blocks}
    }
    pub fn new_empty(blocks_width: &usize, blocks_height: &usize) -> FigureDir {
        FigureDir{height: *blocks_height,
                  width: *blocks_width,
                  blocks: vec![vec![0; *blocks_width]; *blocks_height]}
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_block(&self, x: usize, y: usize) -> u8 {
        self.blocks[y][x]
    }
}

impl Figure {
    pub fn new(name: String) -> Figure {
        Figure{figure_name: name, dir: vec![]}
    }

    pub fn new_from_face(name: String, face_blocks: Vec<Vec<u8>>) -> Figure {
        let mut fig = Figure::new(name);
        let mut dir = FigureDir::new(face_blocks);
        fig.dir.push(dir.clone());
        for _ in 0..3 {
            let mut next_dir = FigureDir::new_empty(&dir.height, &dir.width);
            for y in 0..dir.height {
                for x in 0..dir.width {
                    next_dir.blocks[x][y] = dir.blocks[y][x];
                }
            }
            fig.dir.push(next_dir.clone());
            dir = next_dir;
        }
        fig
    }

    pub fn get_name(&self) -> &String {
        &self.figure_name
    }
    pub fn add_direction(&mut self, dir_blocks: Vec<Vec<u8>>) {
        self.dir.push(FigureDir::new(dir_blocks));
    }
    pub fn get_fig_dir(&self, dir_index: usize) -> FigureDir {
        let dir_index = dir_index % self.dir.len();
        return self.dir[dir_index].clone();
    }

    pub fn place_figure(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                let pos_x = pos.get_x() + col as i32;
                if b != 0 && pf.contains(pos_x, pos_y) {
                    pf.set_block(pos_x as usize, pos_y as usize, b);
                }
            }
        }
    }

    pub fn remove_figure(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.blocks[row][col];
                let pos_x = pos.get_x() + col as i32;
                if b != 0 && pf.contains(pos_x, pos_y) {
                    pf.set_block(pos_x as usize, pos_y as usize, 0);
                }
            }
        }
    }

    pub fn test_figure(&self, pf: &Playfield, pos: &Position) -> bool {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let offs_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let offs_x = pos.get_x() + col as i32;
                let b = fig_dir.blocks[row][col];
                if b != 0 {
                    if !pf.contains(offs_x, offs_y) {
                        return false;
                    }
                    else if pf.block_is_set(offs_x as usize, offs_y as usize) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn move_figure(&self, pf: &mut Playfield,
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

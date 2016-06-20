use block::*;
use figure_dir::*;
use playfield::*;
use position::*;


#[derive(Clone, Debug)]
pub struct Figure {
    figure_name: String,
    pub dir: Vec<FigureDir>,
}


impl Figure {
    pub fn new(name: &str) -> Figure {
        Figure{figure_name: name.to_owned(), dir: vec![]}
    }

    //
    // Build new figure by rotating the face of a figure 90 degrees
    //
    pub fn new_from_face(name: &str,
                         face_blocks: Vec<Vec<u8>>) -> Figure {
        let mut fig = Figure::new(name);
        let mut dir = FigureDir::new(face_blocks);
        fig.dir.push(dir.clone());
        for _ in 0..3 {
            let mut next_dir = FigureDir::new_empty(&dir.height, &dir.width);
            for y in 0..dir.get_height() {
                for x in 0..dir.get_width() {
                    next_dir.blocks[x][y] =
                        dir.blocks[dir.get_height() - y - 1][x].clone();
                }
            }
            if !fig.test_dir_present(&next_dir) {
                fig.dir.push(next_dir.clone());
            }
            dir = next_dir;
        }
        println!("Built figure {} with {} directions",
                 fig.get_name(), fig.get_num_dirs());
        fig
    }

    pub fn get_name(&self) -> &String {
        &self.figure_name
    }
    pub fn get_num_dirs(&self) -> usize {
        return self.dir.len();
    }
    fn test_dir_present(&self, fig_dir: &FigureDir) -> bool {
        for dir in &self.dir {
            if *dir == *fig_dir {
                return true;
            }
        }
        return false;
    }
    pub fn add_dir_face(&mut self, dir_blocks: Vec<Vec<u8>>) {
        self.dir.push(FigureDir::new(dir_blocks));
    }
    pub fn get_fig_dir(&self, dir_index: usize) -> &FigureDir {
        let dir_index = dir_index % self.dir.len();
        return &self.dir[dir_index];
    }

    //
    // Place figure in playfield
    //
    pub fn place(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    pf.set_block(pos_x as usize, pos_y as usize,
                                 b.clone());
                }
            }
        }
    }

    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let mut b = fig_dir.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    let mut b = b.clone();
                    b.locked = true;
                    pf.set_block(pos_x as usize, pos_y as usize, b);
                }
            }
        }
    }

    //
    // Remove figure from playfield
    //
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let b = fig_dir.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    pf.clear_block(pos_x as usize, pos_y as usize);
                }
            }
        }
    }

    //
    // Test if figure can be placed in playfield at position
    // Returns false if not, true if it can be placed.
    //
    pub fn test(&self, pf: &Playfield, pos: &Position) -> bool {
        let fig_dir = self.get_fig_dir(pos.get_dir() as usize);
        for row in 0..fig_dir.blocks.len() {
            let offs_y = pos.get_y() + row as i32;
            for col in 0..fig_dir.blocks[row].len() {
                let offs_x = pos.get_x() + col as i32;
                let b = fig_dir.get_block(col, row);
                if b.is_set() {
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

    //
    // Move a figure in playfield from position to position.
    // Remove from current position, if the new position is valid then
    // place figure there, else put it back on previous position.
    //
    pub fn move_fig(&self, pf: &mut Playfield, current_pos: &Position,
                    new_pos: &Position) -> bool {
        self.remove(pf, current_pos);
        if self.test(pf, new_pos) {
            self.place(pf, new_pos);
            return true;
        }
        self.place(pf, current_pos);
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use figure_dir::*;

    #[test]
    fn test_figure1() {
        let fig = Figure::new_from_face("Figure 1",
                                        vec![vec![0, 0, 0],
                                             vec![1, 1, 1],
                                             vec![0, 1, 0]]);
        assert_eq!(fig.get_num_dirs(), 4);
        assert_eq!(fig.dir[0], FigureDir::new(vec![vec![0, 0, 0],
                                                   vec![1, 1, 1],
                                                   vec![0, 1, 0]]));
        assert_eq!(fig.dir[1], FigureDir::new(vec![vec![0, 1, 0],
                                                   vec![1, 1, 0],
                                                   vec![0, 1, 0]]));
        assert_eq!(fig.dir[2], FigureDir::new(vec![vec![0, 1, 0],
                                                   vec![1, 1, 1],
                                                   vec![0, 0, 0]]));
        assert_eq!(fig.dir[3], FigureDir::new(vec![vec![0, 1, 0],
                                                   vec![0, 1, 1],
                                                   vec![0, 1, 0]]));
    }
    #[test]
    fn test_figure2() {
        let fig = Figure::new_from_face("Figure 2",
                                        vec![vec![0, 1, 0],
                                             vec![0, 1, 0],
                                             vec![0, 1, 0],
                                             vec![0, 1, 0]]);
        assert_eq!(fig.get_num_dirs(), 2);
        assert_eq!(fig.dir[0], FigureDir::new(vec![vec![0, 1, 0],
                                                   vec![0, 1, 0],
                                                   vec![0, 1, 0],
                                                   vec![0, 1, 0]]));
        assert_eq!(fig.dir[1], FigureDir::new(vec![vec![0, 0, 0, 0],
                                                   vec![1, 1, 1, 1],
                                                   vec![0, 0, 0, 0]]));
    }
    #[test]
    fn test_figure3() {
        let fig = Figure::new_from_face("Figure 3",
                                        vec![vec![1, 0],
                                             vec![1, 1],
                                             vec![0, 1]]);
        assert_eq!(fig.get_num_dirs(), 2);
        assert_eq!(fig.dir[0], FigureDir::new(vec![vec![1, 0],
                                                   vec![1, 1],
                                                   vec![0, 1]]));
        assert_eq!(fig.dir[1], FigureDir::new(vec![vec![0, 1, 1],
                                                   vec![1, 1, 0]]));
    }

}

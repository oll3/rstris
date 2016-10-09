use block::*;
use position::Position;
use std::collections::HashSet;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Playfield {
    pf_name: String,
    pf_width: usize,
    pf_height: usize,
    blocks: Vec<Vec<Block>>,
    locked_block: Block,
}

impl Playfield {
    pub fn new(name: &str, width: usize, height: usize) -> Playfield {
        let mut playfield =
            Playfield{pf_name: name.to_owned(),
                      pf_width: width,
                      pf_height: height,
                      blocks: vec![],
                      locked_block: Block::new_locked(0)};
        for _ in 0..height {
            playfield.blocks.push(vec![Block::new_not_set();
                                       width as usize]);
        }
        playfield
    }
    pub fn get_block_id(&self, x: usize, y: usize) -> u8 {
        self.blocks[y][x].id
    }
    pub fn get_block(&self, x: usize, y: usize) -> &Block {
        if x >= self.pf_width || y >= self.pf_height {
            return &self.locked_block;
        }
        return &self.blocks[y][x];
    }
    pub fn width(&self) -> usize {
        self.pf_width
    }
    pub fn height(&self) -> usize {
        self.pf_height
    }
    pub fn contains(&self, pos: &Position) -> bool {
        let x = pos.get_x();
        let y = pos.get_y();
        x >= 0 && x < self.pf_width as i32 &&
            y >= 0 && y < self.pf_height as i32
    }
    pub fn block_state(&self, pos: &Position) -> &BlockState {
        let x = pos.get_x() as usize;
        let y = pos.get_y() as usize;
        &self.get_block(x, y).state
    }
    pub fn block_is_set(&self, pos: &Position) -> bool {
        *self.block_state(pos) != BlockState::NotSet
    }
    pub fn block_is_locked(&self, pos: &Position) -> bool {
        *self.block_state(pos) == BlockState::Locked
    }
    pub fn clear_block(&mut self, pos: &Position) {
        self.blocks[pos.get_y() as usize][pos.get_x() as usize] =
            Block::new_not_set();
    }
    pub fn set_block(&mut self, pos: &Position, block: Block) {
        self.blocks[pos.get_y() as usize][pos.get_x() as usize] = block;
    }

    //
    // Search playfield for full lines (returned in order of top to bottom)
    //
    pub fn get_locked_lines(&self, lines_to_test: &[usize]) -> Vec<usize> {
        let mut full_lines: Vec<usize> = vec![];
        for y in lines_to_test {
            let mut line_full = true;
            for x in 0..self.pf_width {
                let pos = Position::new(x as i32, *y as i32);
                if !self.block_is_locked(&pos) {
                    line_full = false;
                    break;
                }
            }
            if line_full {
                full_lines.push(*y);
            }
        }
        return full_lines;
    }
    pub fn get_all_locked_lines(&self) -> Vec<usize> {
        let mut full_lines: Vec<usize> = vec![];
        for y in 0..self.pf_height {
            let mut line_full = true;
            for x in 0..self.pf_width {
                let pos = Position::new(x as i32, y as i32);
                if !self.block_is_locked(&pos) {
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

    pub fn set_lines(&mut self, lines: &[usize], block: &Block) {
        for line in lines {
            for x in 0..self.pf_width {
                self.blocks[*line][x] = block.clone();
            }
        }
    }

    //
    // Remove a line from playfield and move all lines above downwards
    //
    pub fn throw_line(&mut self, line: usize) {
        let mut y = line as i32;
        while y >= 0 {
            for x in 0..self.pf_width {
                if y >= 1 {
                    self.blocks[y as usize][x] =
                        self.blocks[y as usize - 1][x].clone();
                } else {
                    self.blocks[y as usize][x] = Block::new_not_set();
                }
            }
            y -= 1;
        }
    }

    pub fn count_voids(&self) -> u32 {
        let mut voids = 0;
        let mut visited: HashSet<Position> = HashSet::new();
        let mut all_open: Vec<Position> = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Position::new(x as i32, y as i32);
                if !self.block_is_locked(&pos) {
                    all_open.push(pos);
                }
            }
        }
        for pos in all_open {
            if visited.contains(&pos) {
                continue;
            }
            voids += 1;
            visited.insert(pos.clone());

            let mut fill: Vec<Position> = Vec::new();
            fill.push(pos.clone());
            while fill.len() > 0 {
                let pos = fill.pop().unwrap();
                let test_positions =
                    [Position::new(pos.get_x() + 1, pos.get_y()),
                     Position::new(pos.get_x(), pos.get_y() + 1),
                     Position::new(pos.get_x() - 1, pos.get_y()),
                     Position::new(pos.get_x(), pos.get_y() - 1)];

                for test_pos in test_positions.iter() {
                    if self.contains(test_pos) &&
                        !visited.contains(test_pos) &&
                        !self.block_is_locked(test_pos) {
                            visited.insert(test_pos.clone());
                            fill.push(test_pos.clone());
                        }
                }
            }
        }
        return voids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use block::*;

    #[test]
    fn block_types() {
        // Fill playfiled with locked blocks.
        let pf_height = 22;
        let mut pf = Playfield::new("pf1", 12, pf_height);
        let all_lines = (0..pf_height).collect::<Vec<usize>>();
        pf.set_lines(&all_lines, &Block::new_locked(1));
        assert_eq!(pf.get_locked_lines(&all_lines).len(), pf_height);
        pf.set_lines(&[0], &Block::new_not_set());
        assert_eq!(pf.get_locked_lines(&all_lines)[0], 1);
        pf.set_lines(&all_lines, &Block::new_in_flight(1));
        assert_eq!(pf.get_locked_lines(&all_lines).len(), 0);
    }
    #[test]
    fn throw_lines() {
        // Fill playfiled with locked blocks.
        // Throw away two lines, the top and the bottom
        // and check that the number of locked lines are as
        // expected.
        let pf_height = 22;
        let mut pf = Playfield::new("pf1", 12, pf_height);
        let all_lines = (0..pf_height).collect::<Vec<usize>>();
        pf.set_lines(&all_lines, &Block::new_locked(1));
        pf.throw_line(0);
        pf.throw_line(pf_height - 1);
        assert_eq!(pf.get_locked_lines(&all_lines).len(), pf_height - 2);
        // first locked line is now 2
        assert_eq!(pf.get_locked_lines(&all_lines)[0], 2);
    }
}

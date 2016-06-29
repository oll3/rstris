use block::*;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Playfield {
    pf_name: String,
    pf_width: usize,
    pf_height: usize,
    blocks: Vec<Vec<Block>>,
}


impl Playfield {
    pub fn new(name: &str, width: usize, height: usize) -> Playfield {
        let mut playfield = Playfield{pf_name: name.to_owned(),
                                      pf_width: width,
                                      pf_height: height,
                                      blocks: vec![]};
        for _ in 0..height {
            playfield.blocks.push(vec![Block::new(0); width as usize]);
        }
        playfield
    }
    pub fn get_block_id(&self, x: usize, y: usize) -> u8 {
        self.blocks[y][x].id
    }
    pub fn get_block(&self, x: usize, y: usize) -> &Block {
        &self.blocks[y][x]
    }
    pub fn width(&self) -> usize {
        self.pf_width
    }
    pub fn height(&self) -> usize {
        self.pf_height
    }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.pf_width as i32 &&
            y >= 0 && y < self.pf_height as i32
    }
    pub fn block_is_set(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y).id != 0
    }
    pub fn block_is_locked(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y).id != 0 && self.get_block(x, y).locked
    }
    pub fn clear_block(&mut self, x: usize, y: usize) {
        self.blocks[y][x] = Block::new(0);
    }
    pub fn set_block(&mut self, x: usize, y: usize, block: Block) {
        self.blocks[y][x] = block;
    }

    //
    // Search playfield for full lines (returned in order of top to bottom)
    //
    pub fn get_locked_lines(&self, lines_to_test: &[usize]) -> Vec<usize> {
        let mut full_lines: Vec<usize> = vec![];
        for y in lines_to_test {
            let mut line_full = true;
            for x in 0..self.pf_width {
                if !self.block_is_locked(x, *y) {
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
                    self.blocks[y as usize][x] = Block::new(0);
                }
            }
            y -= 1;
        }
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
        pf.set_lines(&[0], &Block::new(0));
        assert_eq!(pf.get_locked_lines(&all_lines)[0], 1);
        pf.set_lines(&all_lines, &Block::new(1));
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

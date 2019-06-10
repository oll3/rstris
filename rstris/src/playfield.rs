use block::Block;
use matrix2::Matrix2;
use position::Position;

#[derive(Debug, Clone)]
pub struct Playfield {
    pf_name: String,
    blocks: Matrix2<Block>,
    outside_block: Block,
}

impl Playfield {
    pub fn new(name: &str, width: u32, height: u32) -> Playfield {
        Playfield {
            pf_name: name.to_owned(),
            blocks: Matrix2::new(width, height, Block::Clear),
            outside_block: Block::Set(0),
        }
    }
    pub fn get_block(&self, x: i32, y: i32) -> &Block {
        if !self.blocks.contains(x, y) {
            return &self.outside_block;
        }
        return &self.blocks.get(x, y);
    }
    pub fn get_block_by_pos(&self, pos: &Position) -> &Block {
        self.get_block(pos.get_x(), pos.get_y())
    }
    pub fn set_block(&mut self, x: i32, y: i32, block: Block) {
        if self.blocks.contains(x, y) {
            self.blocks.set(x, y, block);
        }
    }
    pub fn set_block_by_pos(&mut self, pos: &Position, block: Block) {
        self.set_block(pos.get_x(), pos.get_y(), block);
    }
    pub fn width(&self) -> u32 {
        self.blocks.width()
    }
    pub fn height(&self) -> u32 {
        self.blocks.height()
    }
    pub fn contains(&self, pos: &Position) -> bool {
        let x = pos.get_x();
        let y = pos.get_y();
        x >= 0 && x < self.blocks.width() as i32 && y >= 0 && y < self.blocks.height() as i32
    }
    pub fn block_is_set(&self, pos: &Position) -> bool {
        self.get_block_by_pos(pos).is_set()
    }
    pub fn clear_block(&mut self, pos: &Position) {
        self.set_block_by_pos(pos, Block::Clear);
    }

    pub fn locked_lines(&self) -> Vec<u32> {
        let mut full_lines: Vec<u32> = vec![];
        self.blocks.line_iter().enumerate().for_each(|(index, l)| {
            if l.into_iter().all(|b| b.is_set()) {
                full_lines.push(index as u32);
            }
        });
        return full_lines;
    }
    pub fn count_locked_lines(&self) -> u32 {
        self.blocks
            .line_iter()
            .filter(|l| l.into_iter().all(|b| b.is_set()))
            .count() as u32
    }

    pub fn set_lines(&mut self, lines: &[u32], block: &Block) {
        for line in lines {
            for x in 0..self.blocks.width() {
                self.set_block(x as i32, *line as i32, block.clone());
            }
        }
    }

    //
    // Remove a line from playfield and move all lines above downwards
    //
    pub fn throw_line(&mut self, line: u32) {
        let mut y = line as i32;
        loop {
            for x in 0..self.blocks.width() as i32 {
                if y >= 1 {
                    let block = self.get_block(x, y - 1).clone();
                    self.set_block(x, y, block);
                } else {
                    self.set_block(x, y, Block::Clear);
                }
            }
            if y == 0 {
                break;
            }
            y -= 1;
        }
    }

    pub fn count_voids(&self) -> u32 {
        let mut voids = 0;
        let mut visited: Matrix2<bool> =
            Matrix2::new(self.blocks.width(), self.blocks.height(), false);
        let mut all_open: Vec<Position> =
            Vec::with_capacity((self.blocks.width() * self.blocks.height()) as usize);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Position::new(x as i32, y as i32);
                if !self.block_is_set(&pos) {
                    all_open.push(pos);
                }
            }
        }
        for pos in all_open {
            if *visited.tv_get(&pos) {
                continue;
            }
            voids += 1;
            visited.tv_set(&pos, true);

            let mut fill: Vec<Position> = Vec::new();
            fill.push(pos.clone());
            while fill.len() > 0 {
                let pos = fill.pop().unwrap();
                let test_positions = [
                    Position::new(pos.get_x() + 1, pos.get_y()),
                    Position::new(pos.get_x(), pos.get_y() + 1),
                    Position::new(pos.get_x() - 1, pos.get_y()),
                    Position::new(pos.get_x(), pos.get_y() - 1),
                ];

                for test_pos in test_positions.iter() {
                    if self.contains(test_pos)
                        && !*visited.tv_get(test_pos)
                        && !self.block_is_set(test_pos)
                    {
                        visited.tv_set(test_pos, true);
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
        let all_lines = (0..pf_height).collect::<Vec<u32>>();
        pf.set_lines(&all_lines, &Block::new_locked(1));
        assert_eq!(pf.locked_lines().len() as u32, pf_height);
        pf.set_lines(&[0], &Block::new_not_set());
        assert_eq!(pf.locked_lines()[0], 1);
        pf.set_lines(&all_lines, &Block::new_in_flight(1));
        assert_eq!(pf.count_locked_lines(), 0);
    }
    #[test]
    fn throw_lines() {
        // Fill playfiled with locked blocks.
        // Throw away two lines, the top and the bottom
        // and check that the number of locked lines are as
        // expected.
        let pf_height = 22;
        let mut pf = Playfield::new("pf1", 12, pf_height);
        let all_lines = (0..pf_height).collect::<Vec<u32>>();
        pf.set_lines(&all_lines, &Block::new_locked(1));
        pf.throw_line(0);
        pf.throw_line(pf_height - 1);
        assert_eq!(pf.count_locked_lines(), pf_height - 2);
        // first locked line is now 2
        assert_eq!(pf.locked_lines()[0], 2);
    }
}

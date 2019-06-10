use crate::block::Block;
use crate::matrix2::Matrix2;
use crate::position::Position;

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
            blocks: Matrix2::from_size(width, height, Block::Clear),
            outside_block: Block::Set(0),
        }
    }
    pub fn get_block(&self, pos: Position) -> &Block {
        if !self.blocks.contains(pos) {
            &self.outside_block
        } else {
            &self.blocks.get(pos)
        }
    }
    pub fn set_block(&mut self, pos: Position, block: Block) {
        if self.blocks.contains(pos) {
            self.blocks.set(pos, block);
        }
    }
    pub fn width(&self) -> u32 {
        self.blocks.width()
    }
    pub fn height(&self) -> u32 {
        self.blocks.height()
    }
    pub fn contains(&self, pos: Position) -> bool {
        self.blocks.contains(pos)
    }
    pub fn block_is_set(&self, pos: Position) -> bool {
        self.get_block(pos).is_set()
    }
    pub fn clear_block(&mut self, pos: Position) {
        self.set_block(pos, Block::Clear);
    }
    pub fn locked_lines(&self) -> Vec<u32> {
        let mut full_lines: Vec<u32> = vec![];
        self.blocks.row_iter().for_each(|row| {
            if row.items.iter().all(|b| b.is_set()) {
                full_lines.push(row.point as u32);
            }
        });
        full_lines
    }
    pub fn count_locked_lines(&self) -> u32 {
        self.blocks
            .row_iter()
            .filter(|row| row.items.iter().all(|b| b.is_set()))
            .count() as u32
    }

    pub fn set_lines(&mut self, lines: &[u32], block: &Block) {
        for line in lines {
            for x in 0..self.blocks.width() {
                self.set_block((x as i32, *line as i32).into(), block.clone());
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
                    let block = self.get_block((x, y - 1).into()).clone();
                    self.set_block((x, y).into(), block);
                } else {
                    self.set_block((x, y).into(), Block::Clear);
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
            Matrix2::from_size(self.blocks.width(), self.blocks.height(), false);
        let mut all_open: Vec<Position> =
            Vec::with_capacity((self.blocks.width() * self.blocks.height()) as usize);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Position::new((x as i32, y as i32));
                if !self.block_is_set(pos) {
                    all_open.push(pos);
                }
            }
        }
        for pos in all_open {
            if *visited.get(pos) {
                continue;
            }
            voids += 1;
            visited.set(pos, true);

            let mut fill: Vec<Position> = Vec::new();
            fill.push(pos);
            while !fill.is_empty() {
                let pos = fill.pop().unwrap();
                let test_positions = [
                    pos + Position { x: 1, y: 0 },
                    pos + Position { x: 0, y: 1 },
                    pos + Position { x: -1, y: 0 },
                    pos + Position { x: 0, y: -1 },
                ];

                for test_pos in test_positions.into_iter() {
                    if self.contains(*test_pos)
                        && !visited.get(*test_pos)
                        && !self.block_is_set(*test_pos)
                    {
                        visited.set(*test_pos, true);
                        fill.push(*test_pos);
                    }
                }
            }
        }
        voids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;

    #[test]
    fn block_types() {
        // Fill playfiled with locked blocks.
        let pf_height = 22;
        let mut pf = Playfield::new("pf1", 12, pf_height);
        let all_lines = (0..pf_height).collect::<Vec<u32>>();
        pf.set_lines(&all_lines, &Block::Set(1));
        assert_eq!(pf.locked_lines().len() as u32, pf_height);
        pf.set_lines(&[0], &Block::Clear);
        assert_eq!(pf.locked_lines()[0], 1);
        pf.set_lines(&all_lines, &Block::Clear);
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
        pf.set_lines(&all_lines, &Block::Set(1));
        pf.throw_line(0);
        pf.throw_line(pf_height - 1);
        assert_eq!(pf.count_locked_lines(), pf_height - 2);
        // first locked line is now 2
        assert_eq!(pf.locked_lines()[0], 2);
    }
}

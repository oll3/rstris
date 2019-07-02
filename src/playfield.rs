use crate::block::Block;
use crate::matrix2::Matrix2;
use crate::vec2::Vec2;

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
    pub fn copy(&mut self, other: &Playfield) {
        if self.height() != other.height() || self.width() != other.width() {
            panic!("can't copy playfield of different sizes");
        }
        self.blocks.clone_from_slice(&other.blocks);
    }
    pub fn get_block(&self, point: Vec2<i32>) -> &Block {
        if !self.blocks.contains(point) {
            &self.outside_block
        } else {
            &self.blocks.get(point)
        }
    }
    pub fn set_block(&mut self, point: Vec2<i32>, block: Block) {
        if self.blocks.contains(point) {
            self.blocks.set(point, block);
        }
    }
    pub fn blocks(&self) -> &Matrix2<Block> {
        &self.blocks
    }
    pub fn width(&self) -> u32 {
        self.blocks.width()
    }
    pub fn height(&self) -> u32 {
        self.blocks.height()
    }
    pub fn contains(&self, point: Vec2<i32>) -> bool {
        self.blocks.contains(point)
    }
    pub fn block_is_set(&self, point: Vec2<i32>) -> bool {
        self.get_block(point).is_set()
    }
    pub fn clear_block(&mut self, point: Vec2<i32>) {
        self.set_block(point, Block::Clear);
    }

    pub fn place(&mut self, point: Vec2<i32>, face: &[(u8, u8, u8)]) {
        for (x, y, id) in face {
            let x = i32::from(*x) + point.x;
            let y = i32::from(*y) + point.y;
            self.blocks.set((x, y).into(), Block::Set(*id));
        }
    }

    pub fn remove(&mut self, point: Vec2<i32>, face: &[(u8, u8, u8)]) {
        for (x, y, _id) in face {
            let x = i32::from(*x) + point.x;
            let y = i32::from(*y) + point.y;
            self.blocks.set((x, y).into(), Block::Clear);
        }
    }

    pub fn test_collision(&self, point: Vec2<i32>, face: &[(u8, u8, u8)]) -> bool {
        for (x, y, _id) in face {
            let x = i32::from(*x) + point.x;
            let y = i32::from(*y) + point.y;
            let block = self.get_block((x, y).into());
            if block.is_set() {
                return true;
            }
        }
        false
    }

    pub fn full_lines(&self) -> impl Iterator<Item = u32> + '_ {
        self.blocks
            .row_iter()
            .enumerate()
            .filter(|(_index_, row)| row.iter().all(|b| b.is_set()))
            .map(|(index, _row)| index as u32)
    }

    pub fn locked_lines(&self) -> Vec<u32> {
        let mut full_lines: Vec<u32> = vec![];
        self.blocks.row_iter().enumerate().for_each(|(index, row)| {
            if row.iter().all(|b| b.is_set()) {
                full_lines.push(index as u32);
            }
        });
        full_lines
    }
    pub fn count_locked_lines(&self) -> u32 {
        self.blocks
            .row_iter()
            .filter(|row| row.iter().all(|b| b.is_set()))
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

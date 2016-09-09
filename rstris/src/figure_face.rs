use block::*;
use playfield::*;
use position::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct FigureFace {
    width: usize,
    height: usize,
    blocks: Vec<Vec<Block>>,
}

impl FigureFace {
    pub fn new(block_ids: &[&[u8]]) -> FigureFace {
        let mut v: Vec<Vec<Block>> = vec![
            vec![Block{id: 0, locked: false};
                 block_ids[0].len()];
            block_ids.len()];
        for row in 0..block_ids.len() {
            for col in 0..block_ids[0].len() {
                v[row][col] = Block::new(block_ids[row][col]);
            }
        }
        return FigureFace{height: v.len(),
                          width: v[0].len(),
                          blocks: v};
    }
    pub fn new_empty(width: usize, height: usize) -> FigureFace {
        FigureFace{width: width, height: height,
                   blocks: vec![vec![Block::new(0); width];
                                height]}
    }
    //
    // Returns a list of row index which contains at least one block
    //
    pub fn get_row_with_blocks(&self) -> Vec<usize> {
        let mut rows_with_blocks: Vec<usize> = Vec::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self.blocks[y][x].is_set() {
                    rows_with_blocks.push(y);
                    break;
                }
            }
        }
        return rows_with_blocks;
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_block_id(&self, x: usize, y: usize) -> u8 {
        self.blocks[y][x].id
    }
    pub fn get_block(&self, x: usize, y: usize) -> &Block {
        &self.blocks[y][x]
    }
    pub fn set_block(&mut self, x: usize, y: usize, block: &Block) {
        self.blocks[y][x] = block.clone();
    }
    pub fn place(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let b = self.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    pf.set_block(pos_x as usize, pos_y as usize,
                                 b.clone());
                }
            }
        }
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let b = self.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    let mut b = b.clone();
                    b.locked = true;
                    pf.set_block(pos_x as usize, pos_y as usize, b);
                }
            }
        }
    }
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let b = self.get_block(col, row);
                let pos_x = pos.get_x() + col as i32;
                if b.is_set() && pf.contains(pos_x, pos_y) {
                    pf.clear_block(pos_x as usize, pos_y as usize);
                }
            }
        }
    }
    pub fn collide_locked(&self, pf: &Playfield, pos: &Position) -> bool {
        // Test for collision with a locked block
        for row in 0..self.blocks.len() {
            let offs_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let offs_x = pos.get_x() + col as i32;
                if self.get_block(col, row).is_set() &&
                    (!pf.contains(offs_x, offs_y) ||
                     pf.block_is_locked(offs_x as usize, offs_y as usize))
                {
                    // Outside playfield is seen as a locked
                    return true;
                }
            }
        }
        return false;
    }
    pub fn collide_blocked(&self, pf: &Playfield, pos: &Position) -> bool {
        for row in 0..self.blocks.len() {
            let offs_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let offs_x = pos.get_x() + col as i32;
                if self.get_block(col, row).is_set() &&
                    (!pf.contains(offs_x, offs_y) ||
                     pf.block_is_set(offs_x as usize, offs_y as usize))
                {
                    return true;
                }
            }
        }
        return false;
    }
}

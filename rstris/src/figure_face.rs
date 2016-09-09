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
    pub fn get_block_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self.blocks[y][x].is_set() {
                    positions.push(Position::new(x as i32, y as i32));
                }
            }
        }
        return positions;
    }
    pub fn get_lowest_block(&self) -> Position {
        let mut pos = Position::new(i32::min_value(), i32::min_value());
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self.blocks[y][x].is_set() && y as i32 > pos.get_y() {
                    pos = Position::new(x as i32, y as i32);
                }
            }
        }
        return pos;
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
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.set_block(&block_pos, b.clone());
                }
            }
        }
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    let mut b = b.clone();
                    b.locked = true;
                    pf.set_block(&block_pos, b);
                }
            }
        }
    }
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.len() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks[row].len() {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.clear_block(&block_pos);
                }
            }
        }
    }
    pub fn collide_locked(&self, pf: &Playfield, pos: &Position) -> bool {
        // Test for collision with a locked block
        for row in 0..self.blocks.len() {
            for col in 0..self.blocks[row].len() {
                let block_pos = Position::new(pos.get_x() + col as i32,
                                              pos.get_y() + row as i32);
                if self.get_block(col, row).is_set() &&
                    (!pf.contains(&block_pos) ||
                     pf.block_is_locked(&block_pos))
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
            for col in 0..self.blocks[row].len() {
                let block_pos = Position::new(pos.get_x() + col as i32,
                                              pos.get_y() + row as i32);
                if self.get_block(col, row).is_set() &&
                    (!pf.contains(&block_pos) ||
                     pf.block_is_set(&block_pos))
                {
                    return true;
                }
            }
        }
        return false;
    }
}

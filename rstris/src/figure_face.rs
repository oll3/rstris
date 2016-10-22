use block::*;
use playfield::*;
use position::*;
use matrix_2d::Matrix2D;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FigureFace {
    blocks: Matrix2D<Block>,
}

impl FigureFace {
    pub fn new(blocks: &[&[Block]]) -> FigureFace {
        FigureFace {
            blocks: Matrix2D::new_init(blocks),
        }
    }
    pub fn new_empty(width: usize, height: usize) -> FigureFace {
        FigureFace {
            blocks: Matrix2D::new(width, height, Block::new_not_set()),
        }
    }
    //
    // Returns a list of row index which contains at least one block
    //
    pub fn get_row_with_blocks(&self) -> Vec<usize> {
        let mut rows_with_blocks: Vec<usize> = Vec::new();
        for y in 0..self.blocks.height() {
            for x in 0..self.blocks.width() {
                if self.blocks.get(x, y).is_set() {
                    rows_with_blocks.push(y);
                    break;
                }
            }
        }
        return rows_with_blocks;
    }
    pub fn get_block_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for y in 0..self.blocks.height() {
            for x in 0..self.blocks.width() {
                if self.blocks.get(x, y).is_set() {
                    positions.push(Position::new(x as i32, y as i32));
                }
            }
        }
        return positions;
    }
    pub fn get_lowest_block(&self) -> Position {
        let mut pos = Position::new(i32::min_value(), i32::min_value());
        for y in 0..self.blocks.height() {
            for x in 0..self.blocks.width() {
                if self.blocks.get(x, y).is_set() && y as i32 > pos.get_y() {
                    pos = Position::new(x as i32, y as i32);
                }
            }
        }
        return pos;
    }
    pub fn get_width(&self) -> usize {
        self.blocks.width()
    }
    pub fn get_height(&self) -> usize {
        self.blocks.height()
    }
    pub fn get_block(&self, x: usize, y: usize) -> &Block {
        &self.blocks.get(x, y)
    }
    pub fn set_block(&mut self, x: usize, y: usize, block: &Block) {
        self.blocks.set(x, y, block.clone());
    }
    pub fn place(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks.width() {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.set_block_by_pos(&block_pos, b.clone());
                }
            }
        }
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks.width() {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    let mut b = b.clone();
                    b.state = BlockState::Locked;
                    pf.set_block_by_pos(&block_pos, b);
                }
            }
        }
    }
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() {
            let pos_y = pos.get_y() + row as i32;
            for col in 0..self.blocks.width() {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col as i32, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.clear_block(&block_pos);
                }
            }
        }
    }
    pub fn test_collision(&self, pf: &Playfield, pos: &Position) -> BlockState {
        let mut state = BlockState::NotSet;
        for row in 0..self.blocks.height() {
            for col in 0..self.blocks.width() {
                let block_pos = Position::new(pos.get_x() + col as i32,
                                              pos.get_y() + row as i32);
                if self.get_block(col, row).is_set() {
                    let pf_block_state =
                        pf.get_block_by_pos(&block_pos).state.clone();
                    match pf_block_state {
                        BlockState::Locked => return pf_block_state.clone(),
                        BlockState::InFlight => state = pf_block_state.clone(),
                        _ => {},
                    }
                }
            }
        }
        return state;
    }
}

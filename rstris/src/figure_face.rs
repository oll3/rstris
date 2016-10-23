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
    pub fn new_empty(width: u32, height: u32) -> FigureFace {
        FigureFace {
            blocks: Matrix2D::new(width, height, Block::new_not_set()),
        }
    }
    //
    // Returns a list of row index which contains at least one block
    //
    pub fn get_row_with_blocks(&self) -> Vec<u32> {
        let mut rows_with_blocks: Vec<u32> = Vec::new();
        for y in 0..self.blocks.height() {
            for x in 0..self.blocks.width() {
                if self.blocks.get(x as i32, y as i32).is_set() {
                    rows_with_blocks.push(y);
                    break;
                }
            }
        }
        return rows_with_blocks;
    }
    pub fn get_block_positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for y in 0..self.blocks.height() as i32 {
            for x in 0..self.blocks.width() as i32 {
                if self.blocks.get(x, y).is_set() {
                    positions.push(Position::new(x, y));
                }
            }
        }
        return positions;
    }
    pub fn get_lowest_block(&self) -> Position {
        let mut pos = Position::new(i32::min_value(), i32::min_value());
        for y in 0..self.blocks.height() as i32 {
            for x in 0..self.blocks.width() as i32 {
                if self.blocks.get(x, y).is_set() && y > pos.get_y() {
                    pos = Position::new(x, y);
                }
            }
        }
        return pos;
    }
    pub fn get_width(&self) -> u32 {
        self.blocks.width()
    }
    pub fn get_height(&self) -> u32 {
        self.blocks.height()
    }
    pub fn get_block(&self, x: i32, y: i32) -> &Block {
        &self.blocks.get(x, y)
    }
    pub fn set_block(&mut self, x: i32, y: i32, block: &Block) {
        self.blocks.set(x, y, block.clone());
    }
    pub fn place(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() as i32 {
            let pos_y = pos.get_y() + row;
            for col in 0..self.blocks.width() as i32 {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.set_block_by_pos(&block_pos, b.clone());
                }
            }
        }
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() as i32 {
            let pos_y = pos.get_y() + row;
            for col in 0..self.blocks.width() as i32 {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    let mut b = b.clone();
                    b.state = BlockState::Locked;
                    pf.set_block_by_pos(&block_pos, b);
                }
            }
        }
    }
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        for row in 0..self.blocks.height() as i32 {
            let pos_y = pos.get_y() + row;
            for col in 0..self.blocks.width() as i32 {
                let b = self.get_block(col, row);
                let block_pos = Position::new(pos.get_x() + col, pos_y);
                if b.is_set() && pf.contains(&block_pos) {
                    pf.clear_block(&block_pos);
                }
            }
        }
    }
    pub fn test_collision(&self, pf: &Playfield, pos: &Position) -> BlockState {
        let mut state = BlockState::NotSet;
        for row in 0..self.blocks.height() as i32 {
            for col in 0..self.blocks.width() as i32 {
                let block_pos = Position::new(pos.get_x() + col,
                                              pos.get_y() + row);
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

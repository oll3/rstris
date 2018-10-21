use block::*;
use matrix2::Matrix2;
use playfield::*;
use position::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FigureFace {
    blocks: Matrix2<Block>,
}

impl FigureFace {
    pub fn new(blocks: &[&[Block]]) -> FigureFace {
        FigureFace {
            blocks: Matrix2::new_init(blocks),
        }
    }
    pub fn new_empty(width: u32, height: u32) -> FigureFace {
        FigureFace {
            blocks: Matrix2::new(width, height, Block::new_not_set()),
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
        self.blocks.iter()
            .filter(|b| b.item.is_set())
            .for_each(|b| {
                let pos = Position::new(pos.get_x() + b.x, pos.get_y() + b.y);
                if pf.contains(&pos) {
                    pf.set_block_by_pos(&pos, b.item.clone());
                }
        });
    }
    pub fn lock(&self, pf: &mut Playfield, pos: &Position) {
        self.blocks.iter()
            .filter(|b| b.item.is_set())
            .for_each(|b| {
                let pos = Position::new(pos.get_x() + b.x, pos.get_y() + b.y);
                if pf.contains(&pos) {
                    let mut b = b.item.clone();
                    b.state = BlockState::Locked;
                    pf.set_block_by_pos(&pos, b);
                }
        });
    }
    pub fn remove(&self, pf: &mut Playfield, pos: &Position) {
        self.blocks.iter()
            .filter(|b| b.item.is_set())
            .for_each(|b| {
                let pos = Position::new(pos.get_x() + b.x, pos.get_y() + b.y);
                if pf.contains(&pos) {
                    pf.clear_block(&pos);
                }
        });
    }
    pub fn test_collision(&self, pf: &Playfield, pos: &Position) -> BlockState {
        self.blocks
            .iter()
            .filter(|b| b.item.is_set())
            .fold(BlockState::NotSet, |acc, b| {
                let pos = Position::new(pos.get_x() + b.x, pos.get_y() + b.y);
                match pf.get_block_by_pos(&pos).state {
                    BlockState::Locked => BlockState::Locked,
                    BlockState::InFlight => {
                        if acc != BlockState::Locked {
                            BlockState::InFlight
                        } else {
                            acc
                        }
                    }
                    _ => acc,
                }
            })
    }
}

use crate::block::*;
use crate::matrix2::Matrix2;
use crate::playfield::*;
use crate::position::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct FigureFace {
    blocks: Matrix2<Block>,
}

impl FigureFace {
    pub fn new(blocks: &[&[Block]]) -> FigureFace {
        FigureFace {
            blocks: Matrix2::from_items(blocks),
        }
    }
    pub fn new_empty(width: u32, height: u32) -> FigureFace {
        FigureFace {
            blocks: Matrix2::from_size(width, height, Block::Clear),
        }
    }

    pub fn row_of_lowest_block(&self) -> i32 {
        let mut lowest = i32::min_value();
        self.blocks.row_iter().for_each(|row| {
            if row.items.iter().any(|b| b.is_set()) && row.point > lowest {
                lowest = row.point;
            }
        });
        lowest
    }
    pub fn width(&self) -> u32 {
        self.blocks.width()
    }
    pub fn height(&self) -> u32 {
        self.blocks.height()
    }
    pub fn get_block(&self, point: Position) -> &Block {
        self.blocks.get(point)
    }
    pub fn set_block(&mut self, point: Position, block: Block) {
        self.blocks.set(point, block);
    }
    pub fn place(&self, pf: &mut Playfield, pos: Position) {
        pf.place(pos, &self.blocks);
    }
    pub fn remove(&self, pf: &mut Playfield, pos: Position) {
        pf.remove(pos, &self.blocks);
    }
    pub fn test_collision(&self, pf: &Playfield, pos: Position) -> bool {
        pf.test_collision(pos, &self.blocks)
    }
}

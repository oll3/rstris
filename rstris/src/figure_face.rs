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
    //
    // Returns a list of row index which contains at least one block
    //
    pub fn rows_with_blocks(&self) -> Vec<u32> {
        let mut rows: Vec<u32> = Vec::new();
        self.blocks.row_iter().for_each(|row| {
            if row.items.into_iter().any(|b| b.is_set()) {
                rows.push(row.point as u32)
            }
        });
        return rows;
    }
    pub fn row_of_lowest_block(&self) -> i32 {
        let mut lowest = i32::min_value();
        self.blocks.row_iter().for_each(|row| {
            if row.items.into_iter().any(|b| b.is_set()) {
                if row.point > lowest {
                    lowest = row.point;
                }
            }
        });
        return lowest;
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
        self.blocks
            .iter()
            .filter(|b| b.item.is_set())
            .for_each(|b| {
                let block_pos = pos + b.point;
                if pf.contains(block_pos) {
                    pf.set_block(block_pos, b.item.clone());
                }
            });
    }
    pub fn remove(&self, pf: &mut Playfield, pos: Position) {
        self.blocks
            .iter()
            .filter(|b| b.item.is_set())
            .for_each(|b| {
                let block_pos = pos + b.point;
                if pf.contains(block_pos) {
                    pf.clear_block(block_pos);
                }
            });
    }
    pub fn test_collision(&self, pf: &Playfield, pos: Position) -> bool {
        self.blocks
            .iter()
            .filter(|b| b.item.is_set())
            .fold(false, |acc, b| {
                let block_pos = pos + b.point;
                match pf.get_block(block_pos) {
                    Block::Set(_) => true,
                    Block::Clear => acc,
                }
            })
    }
}

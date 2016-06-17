use block::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct FigureDir {
    pub width: usize,
    pub height: usize,
    pub blocks: Vec<Vec<Block>>,
}

impl FigureDir {
    pub fn new(dir_block_ids: Vec<Vec<u8>>) -> FigureDir {
        let mut v: Vec<Vec<Block>> = vec![vec![Block{id: 0, locked: false};
                                               dir_block_ids[0].len()];
                                          dir_block_ids.len()];
        for row in 0..dir_block_ids.len() {
            for col in 0..dir_block_ids[0].len() {
                v[row][col] = Block::new(dir_block_ids[row][col]);
            }
        }
        println!("Created Figure dir {:?}", v);
        return FigureDir{height: v.len(),
                         width: v[0].len(),
                         blocks: v};
    }
    pub fn new_empty(blocks_width: &usize, blocks_height: &usize) -> FigureDir {
        FigureDir{width: *blocks_width, height: *blocks_height,
                  blocks: vec![vec![Block::new(0); *blocks_width];
                               *blocks_height]}
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
    pub fn get_block(&self, x: usize, y: usize) -> Block {
        self.blocks[y][x].clone()
    }
}

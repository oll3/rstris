#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Block {
    pub id: u8,
    pub locked: bool,
}

impl Block {
    pub fn new(block_id: u8) -> Block {
        Block{id: block_id, locked: false}
    }

    pub fn is_set(&self) -> bool {
        self.id != 0
    }
}

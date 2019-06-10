#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block {
    Set(u8),
    Clear,
}

impl Block {
    pub fn is_set(&self) -> bool {
        match self {
            Block::Set(_) => true,
            _ => false,
        }
    }
}

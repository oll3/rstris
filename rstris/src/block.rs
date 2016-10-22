#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockState {
    // Not set
    NotSet,

    // Soft might belong to a figure in flight
    InFlight,

    // Hard is all blocks that are not currently in flight
    Locked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub id: u8,
    pub state: BlockState,
}

impl Block {
    pub fn new_in_flight(block_id: u8) -> Block {
        Block{id: block_id, state: BlockState::InFlight}
    }
    pub fn new_locked(block_id: u8) -> Block {
        Block{id: block_id, state: BlockState::Locked}
    }
    pub fn new_not_set() -> Block {
        Block{id: 0, state: BlockState::NotSet}
    }

    pub fn is_set(&self) -> bool {
        self.state != BlockState::NotSet
    }
}

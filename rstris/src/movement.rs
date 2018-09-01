use std::cmp::Ordering;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    RotateCW,
    RotateCCW,
}

#[derive(Debug, Clone)]
pub struct MoveAndTime {
    pub movement: Movement,
    pub time: u64,
}
impl Ord for MoveAndTime {
    fn cmp(&self, other: &MoveAndTime) -> Ordering {
        other.time.cmp(&self.time)
    }
}
impl PartialOrd for MoveAndTime {
    fn partial_cmp(&self, other: &MoveAndTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for MoveAndTime {}
impl PartialEq for MoveAndTime {
    fn eq(&self, other: &MoveAndTime) -> bool {
        self.time == other.time
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Movement {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    RotateCW,
    RotateCCW,
}

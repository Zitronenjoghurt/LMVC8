#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsoleInput {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Touch((u8, u8)),
}

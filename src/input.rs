use crate::Player;

#[derive(Debug)]
pub struct Input {
    pub pos: Position,
    pub player: Player,
}

#[derive(Debug)]
pub enum Position {
    A0,
    A1,
    A2,
    B0,
    B1,
    B2,
    C0,
    C1,
    C2,
}

impl Position {
    pub fn index(&self) -> usize {
        match self {
            Position::A0 => 0,
            Position::A1 => 1,
            Position::A2 => 2,
            Position::B0 => 3,
            Position::B1 => 4,
            Position::B2 => 5,
            Position::C0 => 6,
            Position::C1 => 7,
            Position::C2 => 8,
        }
    }
}

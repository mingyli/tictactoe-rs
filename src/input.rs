use crate::Player;

#[derive(Debug)]
pub struct Input {
    pub position: Position,
    pub player: Player,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    A0 = 0,
    B0 = 1,
    C0 = 2,
    A1 = 3,
    B1 = 4,
    C1 = 5,
    A2 = 6,
    B2 = 7,
    C2 = 8,
}

impl Position {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

use std::convert::TryFrom;
impl TryFrom<String> for Position {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().trim() {
            "A0" => Ok(Position::A0),
            "A1" => Ok(Position::A1),
            "A2" => Ok(Position::A2),
            "B0" => Ok(Position::B0),
            "B1" => Ok(Position::B1),
            "B2" => Ok(Position::B2),
            "C0" => Ok(Position::C0),
            "C1" => Ok(Position::C1),
            "C2" => Ok(Position::C2),
            _ => Err("failed to parse"),
        }
    }
}

#[test]
fn test_convert_from_string() {
    let result = Position::try_from("a0".to_string());
    assert_eq!(result, Ok(Position::A0));
}

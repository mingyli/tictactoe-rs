use crate::{Player, State};

#[derive(Debug, PartialEq)]
pub enum Phase {
    InProgress(State),
    Win(State, Player),
    Tie(State),
}

use std::ops::Try;
impl Try for Phase {
    type Ok = State;
    type Error = (State, Option<Player>);

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Phase::InProgress(state) => Ok(state),
            Phase::Win(state, winner) => Err((state, Some(winner))),
            Phase::Tie(state) => Err((state, None)),
        }
    }

    fn from_error((state, player): Self::Error) -> Self {
        match player {
            None => Phase::Tie(state),
            Some(winner) => Phase::Win(state, winner),
        }
    }

    fn from_ok(v: Self::Ok) -> Self {
        Phase::InProgress(v)
    }
}

use std::fmt;
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::InProgress(state) => {
                writeln!(f, "In progress:")?;
                writeln!(f, "{}", state)?;
            }
            Phase::Tie(state) => {
                writeln!(f, "Tie:")?;
                writeln!(f, "{}", state)?;
            }
            Phase::Win(state, winner) => {
                writeln!(f, "Winner: {}", winner)?;
                writeln!(f, "{}", state)?;
            }
        }
        Ok(())
    }
}

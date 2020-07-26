use crate::{Player, State};

#[derive(Debug, PartialEq)]
pub enum Phase {
    InProgress(State),
    End(State, Option<Player>),
}

use std::ops::Try;
impl Try for Phase {
    type Ok = State;
    type Error = (State, Option<Player>);

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Phase::InProgress(state) => Ok(state),
            Phase::End(state, winner) => Err((state, winner)),
        }
    }

    fn from_error((state, winner): Self::Error) -> Self {
        // If win condition
        Phase::End(state, winner)
    }

    fn from_ok(v: Self::Ok) -> Self {
        Phase::InProgress(v)
    }
}

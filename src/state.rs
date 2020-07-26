use crate::{Input, Phase, Player};

#[derive(Debug, Default, PartialEq)]
pub struct State {
    board: [Option<Player>; 9],
}

impl State {
    pub fn transition(mut self, input: &Input) -> Phase {
        // validation for alternating turns should occur at a higher level
        println!("{}", self);
        println!("Applying {:?}", input);
        self.board[input.pos.index()] = Some(input.player);

        // check win condition
        if let Some(winner) = self.winner() {
            Phase::End(self, Some(winner))
        } else {
            Phase::InProgress(self)
        }
    }

    // Return winner or None for tie
    fn winner(&self) -> Option<Player> {
        None
    }

    pub fn initial() -> State {
        State::default()
    }
}
use std::fmt;
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  a b c")?;
        for (row_number, row) in self.board.chunks(3).enumerate() {
            write!(f, "{} ", row_number)?;
            for cell in row {
                match cell {
                    Some(player) => write!(f, "{}", player)?,
                    None => write!(f, " ")?,
                };
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

use crate::{Input, Phase, Player, Position};

#[derive(Debug, Default, PartialEq)]
pub struct State {
    board: [Option<Player>; 9],
}

impl State {
    fn apply_input(&mut self, input: &Input) {
        self.board[input.position.index()] = Some(input.player);
    }

    pub fn transition(mut self, input: &Input) -> Phase {
        self.apply_input(input);

        if let Some(winner) = self.winner() {
            Phase::Win(self, winner)
        } else if self.board.iter().all(Option::is_some) {
            Phase::Tie(self)
        } else {
            Phase::InProgress(self)
        }
    }

    fn winner(&self) -> Option<Player> {
        const WINNING_ARRANGEMENTS: [[Position; 3]; 8] = [
            [Position::A0, Position::A1, Position::A2],
            [Position::B0, Position::B1, Position::B2],
            [Position::C0, Position::C1, Position::C2],
            [Position::A0, Position::B0, Position::C0],
            [Position::A1, Position::B1, Position::C1],
            [Position::A2, Position::B2, Position::C2],
            [Position::A0, Position::B1, Position::C2],
            [Position::A2, Position::B1, Position::C0],
        ];
        WINNING_ARRANGEMENTS.iter().find_map(|seq| {
            let marks: Option<Vec<Player>> =
                seq.iter().map(|pos| self.board[pos.index()]).collect();
            match marks {
                None => None,
                Some(marks) => {
                    let player = marks[0];
                    if marks.iter().all(|&mark| mark == player) {
                        Some(player)
                    } else {
                        None
                    }
                }
            }
        })
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

#[test]
fn test_state_transitions() {
    use crate::Position;
    let inputs = [
        Input {
            position: Position::A0,
            player: Player::X,
        },
        Input {
            position: Position::B0,
            player: Player::O,
        },
        Input {
            position: Position::C0,
            player: Player::X,
        },
    ];
    let end = inputs.iter().try_fold(State::initial(), State::transition);
    assert_eq!(
        end,
        Phase::InProgress(State {
            board: [
                Some(Player::X),
                Some(Player::O),
                Some(Player::X),
                None,
                None,
                None,
                None,
                None,
                None
            ]
        })
    );
}

#[test]
fn test_winner() {
    let state = State {
        board: [
            Some(Player::X),
            Some(Player::O),
            Some(Player::X),
            None,
            Some(Player::X),
            None,
            None,
            None,
            None,
        ],
    };
    assert_eq!(state.winner(), None);
    let state = State {
        board: [
            Some(Player::X),
            Some(Player::O),
            Some(Player::X),
            None,
            Some(Player::X),
            None,
            None,
            None,
            Some(Player::X),
        ],
    };
    assert_eq!(state.winner(), Some(Player::X));
}

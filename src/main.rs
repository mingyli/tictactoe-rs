#![feature(try_trait)]

mod input;
mod phase;
mod player;
mod state;

use input::{Input, Position};
use phase::Phase;
use player::Player;
use state::State;

fn main() {
    let inputs = [
        Input {
            pos: Position::A0,
            player: Player::X,
        },
        Input {
            pos: Position::A1,
            player: Player::O,
        },
        Input {
            pos: Position::A2,
            player: Player::X,
        },
    ];
    let end = inputs.iter().try_fold(State::initial(), State::transition);
    match end {
        Phase::End(state, winner) => println!("End\n{}\nWinner: {:?}", state, winner),
        Phase::InProgress(state) => println!("InProgress\n{}", state),
    }
}

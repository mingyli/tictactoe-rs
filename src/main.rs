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
    fn read_position() -> Position {
        use std::convert::TryFrom;
        use std::io;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        Position::try_from(buffer).unwrap()
    }

    let state = State::initial();
    let mut phase = Phase::InProgress(state);
    let mut player = Player::X;
    while let Phase::InProgress(state) = phase {
        println!("{}", state);
        println!("Enter position for player {}: ", player);
        let position = read_position();
        let input = Input { position, player };
        player = player.other();
        println!("Applying {:?}", input);
        phase = State::transition(state, &input);
    }
    println!("{}", phase);
}

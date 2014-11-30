#![feature(phase)]
#[phase(plugin, link)] extern crate log;
#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::board;
use tic_tac_toe::line::Marker;
use tic_tac_toe::players::ai;
use tic_tac_toe::players::Player;


#[test]
fn ai_can_be_created_with_a_mark() {
    let player = ai::new(Marker::X);
    let board = board::empty();

    let result = player.make_move(board);

    println!("{}", result);
}



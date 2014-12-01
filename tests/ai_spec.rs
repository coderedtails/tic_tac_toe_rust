#![feature(phase)]
#[phase(plugin, link)] extern crate log;
#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::board;
use tic_tac_toe::board::Board;
use tic_tac_toe::line::Marker;
use tic_tac_toe::players::ai;
use tic_tac_toe::players::Player;


#[test]
fn ai_can_be_created_with_a_mark() {
    let player = ai::new(Marker::X);
    let board = board::empty();

    let expected = Board { marks: [Marker::X, Marker::Empty, Marker::Empty,
                                   Marker::Empty, Marker::Empty, Marker::Empty,
                                   Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.make_move(board);
    assert_eq!(result, expected);
}

#[ignore]
#[test]
fn ai_can_block_in_the_first_row() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::O,
                                Marker::Empty, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let expected = Board { marks: [Marker::O, Marker::X, Marker::O,
                                   Marker::Empty, Marker::Empty, Marker::Empty,
                                   Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.make_move(board);
    assert_eq!(result, expected);
}

#[test]
fn ai_find_the_best_move_itself() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::O,
                                Marker::Empty, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.best_move(board);
    assert_eq!(result, 1u);
}


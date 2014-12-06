extern crate tic_tac_toe;

#[cfg(test)]
extern crate test;

#[cfg(test)]
use test::Bencher;

use tic_tac_toe::core::board;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::Player;

static AI: Ai = Ai {name: Marker::X};

#[bench]
fn benchmark_the_first_move(b: &mut  Bencher) {
    let empty_board = board::empty();
    b.iter(|| AI.make_move(empty_board) );
}

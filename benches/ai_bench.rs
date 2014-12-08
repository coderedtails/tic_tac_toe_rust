#[cfg(test)]
extern crate tic_tac_toe;
extern crate test;

use test::Bencher;

use tic_tac_toe::core::board;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::ai;
use tic_tac_toe::players::Player;

#[bench]
fn benchmark_the_first_move(b: &mut  Bencher) {
    let ai = ai::new(Marker::X);
    let empty_board = board::empty();
    b.iter(|| ai.make_move(empty_board) );
}

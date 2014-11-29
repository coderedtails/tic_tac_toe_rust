#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;

#[test]
fn new_board_should_not_be_finished() {
    let board = tic_tac_toe::board::empty();
    assert!(!tic_tac_toe::board::is_finished(board));
}

#[test]
fn board_with_three_x_in_top_should_be_finished() {
    let board = [Player::X, Player::X, Player::X,
                 Player::Empty, Player::Empty, Player::Empty,
                 Player::Empty, Player::Empty, Player::Empty];

    assert!(tic_tac_toe::board::is_finished(board));
}

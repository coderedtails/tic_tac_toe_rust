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

#[test]
fn board_with_three_x_in_first_column_should_be_finished() {
    let board = [Player::X, Player::Empty, Player::Empty,
                 Player::X, Player::Empty, Player::Empty,
                 Player::X, Player::Empty, Player::Empty];

    assert!(tic_tac_toe::board::is_finished(board));
}

#[test]
fn board_with_three_x_in_first_diagonal_should_be_finished() {
    let board = [Player::X, Player::Empty, Player::Empty,
                 Player::Empty, Player::X, Player::Empty,
                 Player::Empty, Player::Empty, Player::X];

    assert!(tic_tac_toe::board::is_finished(board));
}

#[test]
fn board_full_board_without_winner_is_finished() {
    let board = [Player::X, Player::X, Player::O,
                 Player::O, Player::O, Player::X,
                 Player::X, Player::X, Player::O];

    assert!(tic_tac_toe::board::is_finished(board));
}

#[test]
fn amount_of_remaining_moves_matches_number_of_empty() {
    let board = tic_tac_toe::board::empty();
    let remaining_moves = tic_tac_toe::board::remaining_moves(board);
    assert!(remaining_moves.len() == 9)

}

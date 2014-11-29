#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;

#[test]
fn new_board_should_not_be_finished() {
    let board = tic_tac_toe::board::empty();
    assert!(!board.is_finished());
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
fn full_board_with_draw_is_finished() {
    let board = [Player::X, Player::X, Player::O,
                 Player::O, Player::O, Player::X,
                 Player::X, Player::X, Player::O];

    assert!(tic_tac_toe::board::is_finished(board));
    assert!(tic_tac_toe::board::has_draw(board));
}

#[test]
fn amount_of_remaining_moves_matches_number_of_empty() {
    let board = tic_tac_toe::board::empty();
    let remaining_moves = board.remaining_moves();
    assert_eq!(remaining_moves, vec![0u,1,2,3,4,5,6,7,8])
}

#[test]
fn no_remaining_moves_when_the_board_is_full() {
    let board = [Player::X, Player::X, Player::O,
                 Player::O, Player::O, Player::X,
                 Player::X, Player::X, Player::O];
    let remaining_moves = tic_tac_toe::board::remaining_moves(board);
    assert!(remaining_moves.is_empty())
}

#[test]
fn making_a_move_return_a_fresh_copy() {
    let board = tic_tac_toe::board::empty();
    let changed_board = board.make_move(0, &Player::X);
    let remaining_moves = changed_board.remaining_moves();
    assert!(!remaining_moves.contains(&0u))
}

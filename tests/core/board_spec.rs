#[cfg(test)]

use tic_tac_toe::core::board;
use tic_tac_toe::core::marker::Marker;

#[test]
fn new_board_should_not_be_finished() {
    let board = board::empty();
    assert!(!board.is_finished());
}

#[test]
fn board_with_three_x_in_top_is_finished() {
    let board = ::board_from_str("XXX------");
    assert!(board.is_finished());
}

#[test]
fn board_with_three_x_in_first_column_is_finished() {
    let board = ::board_from_str("X--X--X--");
    assert!(board.is_finished());
}

#[test]
fn board_with_three_x_in_first_diagonal_is_finished() {
    let board = ::board_from_str("X---X---X");
    assert!(board.is_finished());
}

#[test]
fn full_board_with_draw_is_finished() {
    let board = ::board_from_str("XXOOOXXXO");
    assert!(board.is_finished());
    assert!(board.has_draw());
}

#[test]
fn amount_of_remaining_moves_matches_number_of_empty() {
    let board = board::empty();
    let remaining_moves = board.remaining_moves();
    assert_eq!(remaining_moves, vec![0u,1,2,3,4,5,6,7,8])
}

#[test]
fn making_a_move_returns_a_fresh_copy() {
    let board = board::empty();
    let changed_board = board.make_move(0, &Marker::X);
    assert!(!changed_board.remaining_moves().contains(&0u))
    assert!(board.remaining_moves().contains(&0u))
}

#[test]
fn board_with_winner_in_first_row_has_value_7() {
    let board = ::board_from_str("XXX------");
    assert_eq!(7, board.value());
}

#[test]
fn board_with_draw_has_value_0() {
    let board = ::board_from_str("XXOOOXXXO");
    assert_eq!(0, board.value());
}

#[test]
fn a_board_has_multiple_rows() {
    let first:  &[Marker] = &empty_players();
    let second: &[Marker] = &empty_players();
    let third:  &[Marker] = &empty_players();
    let result = vec![first, second, third];
    let board = board::empty();

    assert!(board.rows() == result);
}

fn empty_players() -> [Marker, ..3] {
    [Marker::Empty, Marker::Empty, Marker::Empty]
}

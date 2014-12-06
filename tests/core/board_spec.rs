#[cfg(test)]

use tic_tac_toe::core::board;
use tic_tac_toe::core::marker::Marker;

#[test]
fn new_board_should_not_be_finished() {
    assert!(!board::empty().is_finished());
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
    let remaining_moves = board::empty().remaining_moves();
    for i in range(1u,10) {
        assert!(remaining_moves.contains(&i));
    }
}

#[test]
fn making_a_move_returns_a_fresh_copy() {
    let changed_board = board::empty().make_move(1, &Marker::X);
    assert!(!changed_board.remaining_moves().contains(&1u))
    assert!(board::empty().remaining_moves().contains(&1u))
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

#[cfg(test)]

use tic_tac_toe::core::board::Board;
use tic_tac_toe::core::marker::Marker;

static BOARD: Board = Board{ marks: [Marker::Empty,..9]};

#[test]
fn new_board_should_not_be_finished() {
    assert!(!BOARD.is_finished());
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

#[ignore]
#[test]
fn amount_of_remaining_moves_matches_number_of_empty() {
    let remaining_moves = BOARD.remaining_moves();
    assert_eq!(remaining_moves, vec![1u,2,3,4,5,6,7,8,9])
}

#[test]
fn making_a_move_returns_a_fresh_copy() {
    let changed_board = BOARD.make_move(1, &Marker::X);
    assert!(!changed_board.remaining_moves().contains(&1u))
    assert!(BOARD.remaining_moves().contains(&1u))
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

    assert!(BOARD.rows() == result);
}

#[test]
fn board_exposes_elements_as_map() {
    let elements = BOARD.elements();
    assert!(!elements.contains_key(&0));
    for i in range(1u, 10) {
        match elements.get(&i) {
            Some(n) => assert_eq!(n, &Marker::Empty),
            None => panic!("Missing element {}", i),
        }
    }

}

fn empty_players() -> [Marker, ..3] {
    [Marker::Empty, Marker::Empty, Marker::Empty]
}


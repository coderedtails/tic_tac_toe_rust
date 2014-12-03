#[cfg(test)]

use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::Player;
use tic_tac_toe::core::board::Board;

static BOARD: Board = Board{ marks: [Marker::Empty,..9]};

#[test]
fn applies_move_when_reading_valid_move() {
    let moves = vec!["1".to_string()];
    let human = ::human_with_moves(moves);

    let result = human.make_move(BOARD);
    assert!(!result.remaining_moves().contains(&1));
}

#[test]
fn retries_to_read_move_if_initial_value_is_invalid() {
    let moves = vec!["25".to_string(),
                     "8".to_string()];

    let human = ::human_with_moves(moves);

    let result = human.make_move(BOARD);
    assert!(!result.remaining_moves().contains(&8));
}

#[test]
fn retries_to_read_move_if_initial_value_is_not_a_number() {
    let moves = vec!["foo".to_string(),
                     "6".to_string()];

    let human = ::human_with_moves(moves);

    let result = human.make_move(BOARD);
    assert!(!result.remaining_moves().contains(&6));
}

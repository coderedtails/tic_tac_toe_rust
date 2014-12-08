#[cfg(test)]

use tic_tac_toe::core::board;
use tic_tac_toe::players::Player;

#[test]
fn applies_move_when_reading_valid_move() {
    let human = ::human_with_moves("1");
    let result = human.make_move(board::empty());
    assert!(!result.remaining_moves().contains(&1));
}

#[test]
fn retries_to_read_move_if_initial_value_is_invalid() {
    let human = ::human_with_moves("25,8");
    let result = human.make_move(board::empty());
    assert!(!result.remaining_moves().contains(&8));
}

#[test]
fn retries_to_read_move_if_initial_value_is_not_a_number() {
    let human = ::human_with_moves("foo,6");
    let result = human.make_move(board::empty());
    assert!(!result.remaining_moves().contains(&6));
}

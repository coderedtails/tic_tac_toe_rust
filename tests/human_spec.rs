#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::players::Player;
use tic_tac_toe::core::board;
use tic_tac_toe::io::cli_spy;

#[test]
fn applies_move_when_reading_valid_move() {
    let board = board::empty();
    let moves = vec!["1".to_string()];
    let cli_spy = cli_spy::new_with_moves(moves);

    let display = Display { cli: cli_spy};
    let human = Human { name: Marker::O, display: display };

    let result = human.make_move(board);
    assert!(!result.remaining_moves().contains(&1));
}

#[test]
fn retries_to_read_move_if_initial_value_is_invalid() {
    let board = board::empty();
    let moves = vec!["25".to_string(),
                     "8".to_string()];

    let cli_spy = cli_spy::new_with_moves(moves);

    let display = Display { cli: cli_spy};
    let human = Human { name: Marker::O, display: display };

    let result = human.make_move(board);
    assert!(!result.remaining_moves().contains(&8));
}

#[test]
fn retries_to_read_move_if_initial_value_is_not_a_number() {
    let board = board::empty();
    let moves = vec!["foo".to_string(),
                     "6".to_string()];

    let cli_spy = cli_spy::new_with_moves(moves);

    let display = Display { cli: cli_spy};
    let human = Human { name: Marker::O, display: display };

    let result = human.make_move(board);
    assert!(!result.remaining_moves().contains(&6));
}

#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io;
use tic_tac_toe::line::Player;

#[test]
fn prints_an_empty_row_of_board() {
    let line = [Player::Empty, Player::Empty, Player::Empty];
    let result = io::display::render_line(&line);
    assert_eq!("[0][1][2]".to_string(), result);
}

#[test]
fn prints_player_x() {
    let player = Player::X;
    let result = io::display::render_cell((0u, &player));
    assert_eq!("[X]".to_string(), result);
}

#[test]
fn prints_player_o() {
    let player = Player::O;
    let result = io::display::render_cell((0u,&player));
    assert_eq!("[O]".to_string(), result);
}

#[test]
fn prints_index_when_empty() {
    let player = Player::Empty;
    let result = io::display::render_cell((1u, &player));
    assert_eq!("[1]".to_string(), result);
}

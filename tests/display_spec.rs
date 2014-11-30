#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io;
use tic_tac_toe::line::Player;

#[test]
fn prints_a_board_into_strings_per_row() {
    let first  = "[0][1][2]".to_string();
    let second = "[3][4][5]".to_string();
    let third  = "[6][7][8]".to_string();
    let result: Vec<String> = vec![first, second, third];

    let board = tic_tac_toe::board::empty();
    assert!(io::display::render(board) == result);
}

#[test]
fn prints_an_empty_row_of_board() {
    let line = empty();
    let result = io::display::render_line(&line,0);
    assert_eq!("[0][1][2]".to_string(), result);
}

fn empty() -> [Player, ..3] {
    [Player::Empty, Player::Empty, Player::Empty]
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

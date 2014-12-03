#[cfg(test)]

use tic_tac_toe::io;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::cli_spy::CliSpy;
use tic_tac_toe::core::board;
use tic_tac_toe::core::board::Board;
use tic_tac_toe::core::marker::Marker;

static BOARD: Board = Board{ marks: [Marker::Empty,..9]};

#[test]
fn prints_a_board_to_a_cli_spy() {
    let result  = "[0][1][2]\n[3][4][5]\n[6][7][8]";
    let cli_spy = cli_spy::new();
    let mut display = Display { cli: cli_spy };
    display.render(BOARD);
    assert_printed(&mut display.cli, result);
}

#[test]
fn request_a_valid_move() {
    let cli_spy = cli_spy::new_with_moves(vec!["1".to_string()]);
    let mut display = Display { cli: cli_spy };
    let result = display.request_move();
    assert_eq!(result, 1);
    assert_printed(&mut display.cli, "Choose move");
}

#[test]
fn announces_x_as_the_winner() {
    let cli_spy = cli_spy::new();
    let mut display = Display { cli: cli_spy };
    display.announce_winner(Marker::X);
    assert_printed(&mut display.cli, "The winner was X");
}

#[test]
fn announces_a_draw() {
    let cli_spy = cli_spy::new();
    let mut display = Display { cli: cli_spy };
    display.announce_draw();
    assert_printed(&mut display.cli, "There was a draw");
}

fn assert_printed(cli: &mut CliSpy, line: &str) {
    match cli.last_line() {
        Some(n) => assert_eq!(line.to_string(), n),
        None => panic!("No call to print happend!")
    }
}

#[test]
fn prints_empty_board_into_strings_per_row() {
    let first  = "[0][1][2]".to_string();
    let second = "[3][4][5]".to_string();
    let third  = "[6][7][8]".to_string();
    let result: Vec<String> = vec![first, second, third];

    assert!(io::display::render(BOARD) == result);
}

#[test]
fn prints_non_empty_board_into_strings_per_row() {
    let first  = "[X][1][2]".to_string();
    let second = "[3][4][5]".to_string();
    let third  = "[6][7][O]".to_string();
    let result: Vec<String> = vec![first, second, third];

    let board = board::empty() .make_move(0, &Marker::X).make_move(8, &Marker::O);
    assert!(io::display::render(board) == result);
}

#[test]
fn prints_an_empty_row_of_board() {
    let line = empty();
    let result = io::display::render_line(&line,0);
    assert_eq!("[0][1][2]".to_string(), result);
}

fn empty() -> [Marker, ..3] {
    [Marker::Empty, Marker::Empty, Marker::Empty]
}

#[test]
fn prints_player_x() {
    let input = (0u, &Marker::X);
    let result = io::display::render_cell(input);
    assert_eq!("[X]".to_string(), result);
}

#[test]
fn prints_player_o() {
    let input = (0u, &Marker::O);
    let result = io::display::render_cell(input);
    assert_eq!("[O]".to_string(), result);
}

#[test]
fn prints_index_when_empty() {
    let input = (1u, &Marker::Empty);
    let result = io::display::render_cell(input);
    assert_eq!("[1]".to_string(), result);
}

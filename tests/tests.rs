extern crate tic_tac_toe;

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::cli_spy::CliSpy;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::core::board;
use tic_tac_toe::core::board::Board;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::human;
use tic_tac_toe::players::human::Human;

pub mod core;
pub mod game;
pub mod io;
pub mod players;

pub fn board_from_str(input: &str) -> Board {
    let mut board = board::empty();
    for (idx, c) in input.chars().enumerate() {
        let marker = match c {
                        'X' => Marker::X,
                        'O' => Marker::O,
                         _ => continue,
        };

        board = board.make_move(idx+1, &marker);
    }
    board
}

fn assert_printed(cli: &mut CliSpy, line: &str) {
    match cli.last_line() {
        Some(n) => assert_eq!(line.to_string(), n),
        None => panic!("No call to print happend!")
    }
}

fn create_spy_display() -> Display<CliSpy> {
    let cli_spy = cli_spy::new();
    Display { cli: cli_spy, use_colour: false }
}

fn human_with_moves(moves: &str) -> Human<CliSpy> {
    let cli_spy = cli_spy::new_with_input(moves);
    let display = Display { cli: cli_spy, use_colour: false};
    human::new(Marker::O, display)
}

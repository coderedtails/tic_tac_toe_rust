extern crate tic_tac_toe;

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::cli_spy::CliSpy;
use tic_tac_toe::io::display::Display;

pub mod core;
pub mod game;
pub mod io;
pub mod players;

fn assert_printed(cli: &mut CliSpy, line: &str) {
    match cli.last_line() {
        Some(n) => assert_eq!(line.to_string(), n),
        None => panic!("No call to print happend!")
    }
}

fn create_spy_display() -> Display<CliSpy> {
    let cli_spy = cli_spy::new();
    Display { cli: cli_spy }
}

#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::IO;

#[test]
fn line_with_three_x_has_winner() {
    let mut  spy = cli_spy::new();
    spy.print("Hi there".to_string());
    match spy.last_line() {
        Some(n) => assert_eq!("Hi there".to_string(), n),
        None => panic!("No value"),
    }
}


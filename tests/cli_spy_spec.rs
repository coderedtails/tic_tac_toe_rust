#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::Printer;

#[test]
fn line_with_three_x_has_winner() {
    let spy = cli_spy::new();
    spy.print("Hi there".to_string());
    assert!("Hi there" == spy.last_line().as_slice());
}


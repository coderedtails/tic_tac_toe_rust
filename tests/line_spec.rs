#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Marker;
use tic_tac_toe::line::Winnable;

#[test]
fn line_with_three_x_has_winner() {
    let line  = tic_tac_toe::line::new(Marker::X, Marker::X, Marker::X);
    assert!(line.is_winner(&Marker::X));
    assert!(!line.is_winner(&Marker::O));
}

#[test]
fn empty_line_has_no_winner() {
    let line  = tic_tac_toe::line::empty();
    assert!(line.no_winner());
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = tic_tac_toe::line::new(Marker::X, Marker::O, Marker::X);
    assert!(line.no_winner());
}


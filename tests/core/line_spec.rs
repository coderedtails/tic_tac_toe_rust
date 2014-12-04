#[cfg(test)]

use tic_tac_toe::core::line;
use tic_tac_toe::core::marker::Marker;

#[test]
fn line_with_three_x_has_winner() {
    let line  = line::new(Marker::X, Marker::X, Marker::X);
    assert!(line.is_winner(&Marker::X));
    assert!(!line.is_winner(&Marker::O));
}

#[test]
fn empty_line_has_no_winner() {
    let line  = line::empty();
    assert!(line.no_winner());
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = line::new(Marker::X, Marker::O, Marker::X);
    assert!(line.no_winner());
}


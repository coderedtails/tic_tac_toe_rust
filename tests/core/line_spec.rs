#[cfg(test)]

use tic_tac_toe::core::line::Line;
use tic_tac_toe::core::marker::Marker;

#[test]
fn empty_line_has_no_winner() {
    let line  = Line(None, None, None);
    assert!(line.no_winner());
}

#[test]
fn line_with_three_x_has_winner() {
    let line  = Line(Some(Marker::X),Some(Marker::X), Some(Marker::X));
    assert!(line.is_winner(&Marker::X));
    assert!(!line.is_winner(&Marker::O));
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = Line(Some(Marker::X),Some(Marker::O), Some(Marker::X));
    assert!(line.no_winner());
}

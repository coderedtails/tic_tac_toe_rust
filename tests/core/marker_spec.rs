#[cfg(test)]

use tic_tac_toe::core::marker::Marker;

#[test]
fn opponent_of_x_is_o() {
    assert_eq!(Marker::O, Marker::X.opponent());
}

#[test]
fn opponent_of_o_is_x() {
    assert_eq!(Marker::X, Marker::O.opponent());
}

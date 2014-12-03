#[cfg(test)]

use tic_tac_toe::core::marker::Marker;

#[test]
fn opponent_of_x_is_o() {
    let marker = Marker::X;

    assert_eq!(Marker::O, marker.opponent());
}

#[test]
fn opponent_of_empty_is_empty() {
    let marker = Marker::Empty;

    assert_eq!(marker, marker.opponent());
}

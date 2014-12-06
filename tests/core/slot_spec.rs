#[cfg(test)]

use tic_tac_toe::core::slot::Slot;
use tic_tac_toe::core::marker::Marker;

#[test]
fn slots_with_moves_print_the_number() {
    assert_eq!("3", Slot::Move(3).printable().as_slice());
}

#[test]
fn slots_with_markers_print_the_marker() {
    assert_eq!("X", Slot::Placed(Marker::X).printable().as_slice());
}

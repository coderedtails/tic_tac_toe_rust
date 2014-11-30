#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io::CliSpy;
#[test]
fn line_with_three_x_has_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::X, Player::X);
    assert!(line.is_winner(&Player::X));
    assert!(!line.is_winner(&Player::O));
}


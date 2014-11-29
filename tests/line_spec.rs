#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;
use tic_tac_toe::line::Winnable;
use tic_tac_toe::line;

#[test]
fn line_with_three_x_has_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::X, Player::X);
    assert!(line.is_winner(&Player::X));
    assert!(!line.is_winner(&Player::O));
}

#[test]
fn empty_line_has_no_winner() {
    let line  = tic_tac_toe::line::empty();
    assert!(line.no_winner());
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::O, Player::X);
    assert!(line.no_winner());
}


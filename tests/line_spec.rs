#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;
use tic_tac_toe::line;

#[test]
fn line_with_three_x_has_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::X, Player::X);
    assert!(line::is_winner_of_line(line, Player::X));
    assert!(!line::is_winner_of_line(line, Player::O));
}

#[test]
fn line_with_three_o_has_winner() {
    let line  = tic_tac_toe::line::new(Player::O, Player::O, Player::O);
    assert!(line::is_winner_of_line(line, Player::O));
}

#[test]
fn empty_line_has_no_winner() {
    let line  = tic_tac_toe::line::empty();
    assert!(line::has_no_winner(line));
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::O, Player::X);
    assert!(line::has_no_winner(line));
}

#[test]
fn create_a_from_a_slice_of_players() {
    let player_slice: &[Player] = &[Player::X, Player::X, Player::X];
    let line  = tic_tac_toe::line::of_slice(player_slice);
    assert!(line::is_winner_of_line(line, Player::X));
}

#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;

#[test]
fn lines_have_three_elements() {
    let line = tic_tac_toe::line::new(Player::X, Player::X, Player::X);
}

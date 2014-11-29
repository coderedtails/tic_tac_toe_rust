#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::line::Player;
use tic_tac_toe::line::WinnerResult;
use tic_tac_toe::line::Line;
use tic_tac_toe::line;

#[test]
fn line_with_three_x_has_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::X, Player::X);
    assert!(is_winner_of_line(line, Player::X));
    assert!(!is_winner_of_line(line, Player::O));
}

#[test]
fn line_with_three_o_has_winner() {
    let line  = tic_tac_toe::line::new(Player::O, Player::O, Player::O);
    assert!(is_winner_of_line(line, Player::O));
}

fn is_winner_of_line(line: Line, player: Player) -> bool {
    match line::has_winner(line) {
        WinnerResult::Winner(n) if n == player => true,
                                             _ => false,
    }
}

#[test]
fn empty_line_has_no_winner() {
    let line  = tic_tac_toe::line::empty();
    assert!(has_no_winner(line));
}

fn has_no_winner(line: Line) -> bool {
    match line::has_winner(line) {
        WinnerResult::NoWinner => true,
        _ => false,
    }
}

#[test]
fn mixed_line_has_no_winner() {
    let line  = tic_tac_toe::line::new(Player::X, Player::O, Player::X);
    assert!(has_no_winner(line));
}

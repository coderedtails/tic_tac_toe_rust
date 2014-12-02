#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::players::game_mode::GameMode;
use tic_tac_toe::players::Player;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::players::ai;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli::Cli;

#[test]
fn can_be_created_with_two_ais() {
    let ai = ai::new(Marker::O);

    GameMode { first: ai, second: ai};
}

#[test]
fn can_be_created_with_ai_and_human() {
    let human = basic_human(Marker::X);

    let ai = ai::new(Marker::O);

    GameMode { first: ai, second: human};
}


fn basic_human(name: Marker) -> Human<Cli> {
    Human { name: name, display: Display { cli: Cli }}
}
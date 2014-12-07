#[cfg(test)]

use tic_tac_toe::game::game_mode::GameMode;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::players::ai;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli::Cli;
use std::cell::Cell;

#[test]
fn create_opponents_line_with_players_names() {
    let human = basic_human(Marker::X);
    let ai = ai::new(Marker::O);

    let game_mode = GameMode { first: box human, second: box ai, counter: Cell::new(0u) };
    let line  = game_mode.opponents_line();
    assert_eq!("Human vs. Ai", line.as_slice());
}


fn basic_human(name: Marker) -> Human<Cli> {
    Human { name: name, display: Display { cli: Cli, use_colour: false }}
}

#[cfg(test)]

use tic_tac_toe::players::game_mode;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::players::ai;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli::Cli;

#[test]
fn create_opponents_line_with_players_names() {
    let human = basic_human(Marker::X);
    let ai = ai::new(Marker::O);

    let game_mode = game_mode::new(human,ai);
    let line  = game_mode.opponents_line();
    assert_eq!("Human vs. Ai", line.as_slice());
}


fn basic_human(name: Marker) -> Human<Cli> {
    Human { name: name, display: Display { cli: Cli, use_colour: false }}
}

#[cfg(test)]

extern crate tic_tac_toe;

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::players::ai;
use tic_tac_toe::players::game_mode::GameMode;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::game::game::Game;


#[test]
fn line_with_three_x_has_winner() {
    let cli = cli_spy::new();
    let display =  Display { cli: cli };

    let ai = ai::new(Marker::X);
    let other = ai::new(Marker::O);

    let game_mode = GameMode { first: ai, second: other};

    let game = Game { display: display};
    game.play(game_mode);
}

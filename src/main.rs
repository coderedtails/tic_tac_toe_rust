extern crate tic_tac_toe;

use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode;

fn main() {
    let first =  Ai { name: Marker::X };

    let display = Display { cli: cli::new()};
    let second = Human { name: Marker::O, display: display };
    let game_mode = game_mode::new(second, first);

    let game = Game { display: display};
    game.play(game_mode);
}

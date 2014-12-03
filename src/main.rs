extern crate tic_tac_toe;

use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode;

fn main() {

    let display = Display { cli: cli::new(), use_colour: true};
    let game_mode = game_mode::choose_game_mode(display);

    let game = Game { display: display};
    game.play(game_mode);
}


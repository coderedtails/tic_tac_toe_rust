extern crate tic_tac_toe;

use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode;

fn main() {
    let ai =  Ai { name: Marker::O };
    //let ai2 =  Ai { name: Marker::O };

    let display = Display { cli: cli::new(), use_colour: true};
    let human = Human { name: Marker::X, display: display };
    let game_mode = game_mode::new(human, ai);

    let game = Game { display: display};
    game.play(game_mode);
}

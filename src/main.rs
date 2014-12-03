extern crate tic_tac_toe;

use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli::Cli;
use tic_tac_toe::io::cli;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode::GameMode;

use std::cell::Cell;

fn main() {

    let display = Display { cli: cli::new(), use_colour: true};
    let game_mode = choose_game(display);

    let game = Game { display: display};
    game.play(game_mode);
}


fn choose_game<'a>(display: Display<Cli>) -> GameMode<'a> {
    let ai =  Ai { name: Marker::O };
    let human = Human { name: Marker::X, display: display };
    GameMode { first: box human, second: box ai, counter: Cell::new(0u) }
}


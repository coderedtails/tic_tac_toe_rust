extern crate tic_tac_toe;

use tic_tac_toe::io::cli;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::game::game_mode;
use tic_tac_toe::game::game::Game;

fn main() {
    let display = Display { cli: cli::new(), use_colour: true};
    let game = Game { display: display};
    loop {
        let game_mode = game_mode::choose_game_mode(display);
        game.play(game_mode);
        match display.request_rematch() {
            false => break,
            true => continue,
        }
    }
}

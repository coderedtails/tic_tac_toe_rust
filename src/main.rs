extern crate tic_tac_toe;

use tic_tac_toe::io::cli;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::board;

fn main() {
    let cli = cli::new();
    let display = Display { cli: box cli };
    let board = board::empty();

    display.render(board);
}

extern crate tic_tac_toe;

use tic_tac_toe::core::board;
use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::human::Human;
use tic_tac_toe::players::Player;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::io::display::Display;

fn main() {
    let display = Display;
    let first =  Ai { name: Marker::X };
    let second = Human { name: Marker::O, display: display };
    let players: [&Player,..2] = [&first, &second];
    let mut board = board::empty();

    let mut idx = 0;
    let mut current = players[idx % 2];
    loop {
       board = current.make_move(board);
       display.render(board);
       if board.is_finished() {
           break;
       }
       idx += 1;
       current = players[idx % 2];
    }

}

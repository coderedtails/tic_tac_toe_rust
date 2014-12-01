extern crate tic_tac_toe;

use tic_tac_toe::core::board;
use tic_tac_toe::players::ai::Ai;
use tic_tac_toe::players::Player;
use tic_tac_toe::core::marker::Marker;

fn main() {
    let first =  Ai { name: Marker::X };
    let second = Ai { name: Marker::O };
    let players = [first, second];
    let mut board = board::empty();

    let mut idx = 0;
    let mut current = players[idx % 2];
    loop {
       board = current.make_move(board);
       println!("{}", board);
       if board.is_finished() {
           break;
       }
       idx += 1;
       current = players[idx % 2];
    }

}

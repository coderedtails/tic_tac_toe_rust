use core::board::Board;
use core::marker::Marker;
use players::Player;

use io::display::Display;

pub struct Human {
    pub name: Marker,
    pub display: Display,
}

impl Player for Human {
    fn make_move(&self, board: Board) -> Board{
        let possible_moves = board.remaining_moves();
        let choice = self.display.request_move();

        if possible_moves.contains(&choice) {
            board.make_move(choice, &self.name)
        } else {
            self.make_move(board)
        }
    }
}

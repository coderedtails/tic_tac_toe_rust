use core::board::Board;
use core::marker::Marker;
use players::Player;
use io::IO;

use io::display::Display;

pub struct Human<P> {
    pub name: Marker,
    pub display: Display<P>,
}

impl<P: IO> Player for Human<P> {
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

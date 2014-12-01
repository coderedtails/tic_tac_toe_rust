use core::board::Board;
use core::marker::Marker;
use players::Player;

pub struct Ai {
    name: Marker,
}

impl Player for Ai {
    fn make_move(&self, board: Board) -> Board{
        let best_move = &self.best_move(board);
        board.make_move(*best_move, &self.name)
    }
}

impl Ai {
    pub fn best_move(&self, board: Board) -> uint {
        1u
    }
}

pub fn new(name: Marker) -> Ai {
    Ai { name: name }
}

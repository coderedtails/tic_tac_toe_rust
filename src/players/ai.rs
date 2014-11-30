use board::Board;
use line::Marker;
use players::Player;

pub struct Ai {
    name: Marker,
}

impl Player for Ai {
    fn make_move(&self, board: Board) -> Board{
        board.clone()
    }
}

pub fn new(name: Marker) -> Ai {
    Ai { name: name }
}

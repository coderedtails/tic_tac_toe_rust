use board::Board;
use line::Marker;
use players::Player;

pub struct Ai {
    name: Marker,
}

impl Player for Ai {
    fn make_move(&self, board: Board) -> Board{
        board.make_move(0u, &self.name)
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

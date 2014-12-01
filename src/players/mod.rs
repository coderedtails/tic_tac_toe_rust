use core::board::Board;

pub mod ai;

pub trait Player {
    fn make_move(&self, board: Board) -> Board;
}


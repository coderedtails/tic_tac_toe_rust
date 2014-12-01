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

    fn negamax(board: Board, alpha: int, beta: int, name: Marker) -> int {
        if board.is_finished() {
            Ai::value_of_board(board, name)
        } else {
            1
        }
    }

    fn value_of_board(board: Board, name: Marker) -> int {
      if board.is_winner(&name) {
          board.value()
      } else {
         -board.value()
      }
    }

    fn score_unfinshed(board: Board, alpha: int, beta: int, name: Marker) -> int {
        let best_score = alpha;
        for m in board.remaining_moves().iter() {
        }
        0
    }
}

pub fn new(name: Marker) -> Ai {
    Ai { name: name }
}

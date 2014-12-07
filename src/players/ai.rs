use core::board::Board;
use core::marker::Marker;
use players::Player;

use std::cmp;
use std::num::Int;
use std::rand;
use std::rand::Rng;

pub struct Ai {
    pub name: Marker,
}

pub fn new(name: Marker) -> Ai {
    Ai { name: name }
}

impl Player for Ai {
    fn make_move(&self, board: Board) -> Board{
        let best_move = self.best_move(board);
        board.make_move(best_move, &self.name)
    }

    fn player_type(&self) -> String {
        "Ai".to_string()
    }
}

impl Ai {
    pub fn best_move(&self, board: Board) -> uint {
        let mut best_move = 0u;
        let mut best_score: int = Int::min_value();
        for m in Ai::shuffled(board.remaining_moves()).iter() {
            let new_board = board.make_move(*m, &self.name);
            let score = -Ai::negamax(new_board, best_score, 10, self.name.opponent());

            if score > best_score {
                best_score = score;
                best_move = *m;
            }
        }
        best_move
    }

    fn shuffled(moves: Vec<uint>) -> Vec<uint> {
        let mut v: Vec<uint> = moves;
        {
            let mut slice: &mut [uint] = &mut *v;
            rand::task_rng().shuffle(slice);
        }
        v
    }

    fn negamax(board: Board, alpha: int, beta: int, name: Marker) -> int {
        if board.is_finished() {
            Ai::value_of_board(board, name)
        } else {
            Ai::score_unfinshed(board, alpha, beta, name)
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
        let mut best_score = alpha;
        let mut current_alpha = alpha;
        for m in Ai::shuffled(board.remaining_moves()).iter() {
            let next_board = board.make_move(*m, &name);
            let score = -Ai::negamax(next_board, -beta, -current_alpha, name.opponent());

            best_score = cmp::max(best_score, score);
            current_alpha = cmp::max(current_alpha, score);
            if current_alpha > beta {
                break;
            }
        }
        best_score
    }
}

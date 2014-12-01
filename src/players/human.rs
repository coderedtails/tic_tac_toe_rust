use core::board::Board;
use core::marker::Marker;
use players::Player;

use std::io;

pub struct Human {
    pub name: Marker,
}

impl Player for Human {
    fn make_move(&self, board: Board) -> Board{
        let possible_moves = board.remaining_moves();
        let choice = request_move();

        if possible_moves.contains(&choice) {
            board.make_move(choice, &self.name)
        } else {
            self.make_move(board)
        }
    }
}

fn to_int(input: String) -> uint {
    let raw: Option<uint> = from_str(input.as_slice().trim());

    match raw {
        Some(number) => number,
        None => 100,
    }
}

fn request_move() -> uint {
    println!("Choose move");
    let input = io::stdin().read_line().ok().expect("Failed to read line");
    to_int(input)
}




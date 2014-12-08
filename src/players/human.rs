use core::board::Board;
use core::marker::Marker;
use io::display::Display;
use io::IO;
use players::Player;

pub struct Human<P> {
    name: Marker,
    display: Display<P>,
}

impl<P: IO> Player for Human<P> {
    fn player_type(&self) -> String {
        "Human".to_string()
    }

    fn make_move(&self, board: Board) -> Board{
        let possible_moves = board.remaining_moves();
        let choice = self.choose_move_from(possible_moves);
        board.make_move(choice, &self.name)
    }
}

pub fn new<P>(name: Marker, display: Display<P>) -> Human<P> {
   Human { name: name, display: display }
}

impl<P: IO> Human<P> {
    fn choose_move_from(&self, moves: Vec<uint>) -> uint {
        loop {
            let choice = self.display.request_move();
            if moves.contains(&choice) {
                return choice
            }
        }
    }
}

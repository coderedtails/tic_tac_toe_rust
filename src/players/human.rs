use io::display::Display;
use core::board::Board;
use core::marker::Marker;
use players::Player;
use io::IO;

pub struct Human<P> {
    pub name: Marker,
    pub display: Display<P>,
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

impl<P: IO> Human<P> {
    fn choose_move_from(&self, moves: Vec<uint>) -> uint {
        loop {
            let choice = &self.display.request_move();
            if moves.contains(choice) {
                return *choice
            }
        }
    }
}

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
    fn make_move(&self, board: Board) -> Board{
        let possible_moves = board.remaining_moves();

        loop {
            let choice = self.display.request_move();
            if possible_moves.contains(&choice) {
                return board.make_move(choice, &self.name)
            }
        }
    }

    fn player_type(&self) -> String {
        "Human".to_string()
    }
}

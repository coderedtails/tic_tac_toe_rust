use core::board;
use io::IO;
use io::display::Display;
use players::game_mode::GameMode;
use players::Player;
use core::line::WinnerResult;

pub struct Game<P> {
    pub display: Display<P>,
}

impl <P: IO>Game<P>{
    pub fn play<T: Player, R: Player>(&self, mode: GameMode<T,R>) {
        let mut board = board::empty();

        let mut current = mode.next();
        loop {
            self.display.render(board);
            board = current.make_move(board);
            if board.is_finished() {
                break;
            }
            current = mode.next();
        }

        self.display.render(board);
        match board.winner() {
            WinnerResult::Winner(n) => self.display.announce_winner(n),
            WinnerResult::NoWinner => self.display.announce_draw(),
        }
    }
}

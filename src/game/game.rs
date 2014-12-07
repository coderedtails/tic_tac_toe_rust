use core::board;
use core::board::Board;
use io::IO;
use io::display::Display;
use game::game_mode::GameMode;
use players::Player;
use core::line::WinnerResult;

pub struct Game<P> {
    pub display: Display<P>,
}

impl <P: IO>Game<P>{
    pub fn play(&self, mode: GameMode) {
        let mut board = board::empty();
        let mut current_player = mode.next_player();

        loop {
            self.display.draw(board);
            board = current_player.make_move(board);
            if board.is_finished() {
                break;
            }
            current_player = mode.next_player();
        }

        self.show_result(board);
    }

    fn show_result (&self, board: Board) {
        self.display.draw(board);
        match board.winner() {
            WinnerResult::Winner(n) => self.display.announce_winner(n),
            WinnerResult::NoWinner => self.display.announce_draw(),
        }
    }
}

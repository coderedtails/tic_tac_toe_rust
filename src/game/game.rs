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

        self.display.draw(board);
        self.show_result(board.winner());
    }

    fn show_result (&self, game_result: WinnerResult) {
        match game_result {
            WinnerResult::Winner(n) => self.display.announce_winner(n),
            WinnerResult::NoWinner => self.display.announce_draw(),
        }
    }
}

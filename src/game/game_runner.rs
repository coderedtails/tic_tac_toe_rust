use core::board;
use core::board::Board;
use core::line::WinnerResult;
use game::game_mode::GameMode;
use io::IO;
use io::display::Display;
use players::Player;

pub struct GameRunner<P> {
    pub display: Display<P>,
}

pub fn new<P>(display: Display<P>) -> GameRunner<P> {
    GameRunner { display: display }
}

impl <P: IO>GameRunner<P>{
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
            WinnerResult::Winner(mark) => self.display.announce_winner(mark),
            WinnerResult::NoWinner => self.display.announce_draw(),
        }
    }
}

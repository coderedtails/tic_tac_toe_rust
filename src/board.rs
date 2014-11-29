use line;
use line::Line;
use line::Player;
use line::WinnerResult;

pub fn empty() -> [Player, ..9] {
    [Player::Empty,..9]
}

pub fn is_finished(board: [Player, ..9]) -> bool {
    for column in columns(board).iter() {
        match line::has_winner(*column) {
            WinnerResult::Winner(_) => return true,
            WinnerResult::NoWinner  => continue,
        }
    }
    false
}

fn columns(board: [Player, ..9]) -> [Line,..3] {
    [line::new(board[0], board[1], board[2]),
     line::new(board[3], board[4], board[5]),
     line::new(board[6], board[7], board[8]) ]
}

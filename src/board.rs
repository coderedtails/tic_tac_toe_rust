use line;
use line::Line;
use line::Player;
use line::WinnerResult;

pub fn empty() -> [Player, ..9] {
    [Player::Empty,..9]
}

pub fn is_finished(board: [Player, ..9]) -> bool {
    for column in all_lines(board).into_iter() {
        match line::has_winner_line(column) {
            WinnerResult::Winner(_) => return true,
            WinnerResult::NoWinner  => continue,
        }
    }
    false
}

fn all_lines(board: [Player,..9]) ->Vec<[Player,..3]> {
    let mut lines:Vec<[Player, ..3]> = Vec::new();
    lines.push_all(&rows(board));
    lines.push_all(&columns(board));
    lines.push_all(&diagonals(board));
    lines
}

fn rows(board: [Player, ..9]) -> [[Player, ..3], ..3]  {
    [of(board, 0, 1 ,2),
     of(board, 3, 4 ,5),
     of(board, 6, 7 ,8)]
}

fn columns(board: [Player, ..9]) -> [[Player, ..3], ..3]  {
    [of(board, 0, 3 ,6),
     of(board, 1, 4 ,7),
     of(board, 2, 5 ,8)]
}

fn diagonals(board: [Player, ..9]) -> [[Player, ..3], ..2]  {
    [of(board, 0, 4 ,8),
     of(board, 6, 4 ,2)]
}

fn of(board: [Player, ..9], first: uint, second: uint, third: uint) -> [Player, ..3] {
    [board[first], board[second], board[third]]
}

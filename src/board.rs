use line;
use line::Line;
use line::Player;
use line::WinnerResult;

pub fn empty() -> [Player, ..9] {
    [Player::Empty,..9]
}

pub fn is_finished(board: [Player, ..9]) -> bool {
    has_winner(board) || has_draw(board)
}

pub fn has_draw(board: [Player, ..9]) -> bool {
    !has_winner(board) && remaining_moves(board).is_empty()
}


pub fn remaining_moves(board: [Player,..9]) -> Vec<uint> {
    board.iter().enumerate().filter_map(player_to_index).collect()
}

fn player_to_index(pair: (uint, &Player)) -> Option<uint> {
    let (idx, player) = pair;
    match *player {
        Player::Empty => Some(idx),
        _             => None,
    }
}

fn has_winner(board: [Player, ..9]) -> bool {
    for line in all_lines(board).into_iter() {
        match line::has_winner_line(line) {
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

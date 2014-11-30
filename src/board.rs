use line;
use line::Player;
use line::WinnerResult;
use line::Winnable;

pub struct Board {
    marks: [Player,..9]
}

impl Board {
    pub fn remaining_moves(&self) -> Vec<uint> {
        remaining_moves(&self.marks)
    }

    pub fn is_finished(&self) -> bool {
        has_winner(&self.marks) || has_draw(&self.marks)
    }

    pub fn make_move(&self, location: uint, player: &Player) -> Board  {
        let mut new_marks = self.marks.clone();
        new_marks[location] = *player;
        Board { marks: new_marks }
    }
}

pub fn empty() -> Board {
    Board{ marks: [Player::Empty,..9]}
}

pub fn is_finished(board: &[Player, ..9]) -> bool {
    has_winner(board) || has_draw(board)
}

pub fn has_draw(board: &[Player]) -> bool {
    !has_winner(board) && remaining_moves(board).is_empty()
}

pub fn remaining_moves(board: &[Player]) -> Vec<uint> {
    board.iter().enumerate().filter_map(player_to_index).collect()
}

fn player_to_index(pair: (uint, &Player)) -> Option<uint> {
    let (idx, player) = pair;
    match *player {
        Player::Empty => Some(idx),
        _ => None,
    }
}

fn has_winner(board: &[Player]) -> bool {
    for line in all_lines(board).into_iter() {
        match line.winner() {
            WinnerResult::Winner(_) => return true,
            WinnerResult::NoWinner  => continue,
        }
    }
    false
}

fn all_lines(board: &[Player]) ->Vec<line::Line> {
    let mut lines:Vec<line::Line> = Vec::new();
    lines.push_all(&rows(board));
    lines.push_all(&columns(board));
    lines.push_all(&diagonals(board));
    lines
}

fn rows(board: &[Player]) -> [line::Line, ..3]  {
    [of(board, 0, 1 ,2),
     of(board, 3, 4 ,5),
     of(board, 6, 7 ,8)]
}

fn columns(board: &[Player]) -> [line::Line, ..3]  {
    [of(board, 0, 3 ,6),
     of(board, 1, 4 ,7),
     of(board, 2, 5 ,8)]
}

fn diagonals(board: &[Player]) -> [line::Line, ..2]  {
    [of(board, 0, 4 ,8),
     of(board, 6, 4 ,2)]
}

fn of(board: &[Player], first: uint, second: uint, third: uint) -> line::Line {
    line::new(board[first], board[second], board[third])
}

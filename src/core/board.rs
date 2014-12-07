use core::line;
use core::line::WinnerResult;
use core::marker::Marker;
use core::slot::Slot;


#[deriving(Show, Clone, PartialEq)]
pub struct Board {
    pub marks: [Slot,..9]
}

pub static BOARD_SIZE: uint = 3;

pub fn empty() -> Board {
    Board { marks: [Slot::Move(1), Slot::Move(2), Slot::Move(3),
                    Slot::Move(4), Slot::Move(5), Slot::Move(6),
                    Slot::Move(7), Slot::Move(8), Slot::Move(9)]}
}

impl Board {
    pub fn remaining_moves(&self) -> Vec<uint> {
        self.marks.iter().filter_map(keep_moves).collect()
    }

    pub fn winner(&self) -> WinnerResult {
        for line in all_lines(&self.marks).into_iter() {
            let result = line.winner();
            match result {
                WinnerResult::Winner(_) => return result,
                _ => continue,
            }
        }
        WinnerResult::NoWinner
    }

    pub fn is_winner(&self, player: &Marker) -> bool {
        match self.winner() {
            WinnerResult::Winner(ref n) if n == player => true,
            _ => false,
        }
    }

    fn has_winner(&self) -> bool {
        match self.winner() {
            WinnerResult::Winner(_) => true,
            WinnerResult::NoWinner  => false,
        }
    }

    pub fn make_move(&self, location: uint, player: &Marker) -> Board  {
        let mut new_marks = self.marks.clone();
        new_marks[location-1] = Slot::Placed(*player);
        Board { marks: new_marks }
    }

    pub fn rows(&self) -> Vec<&[Slot]> {
        self.marks.chunks(BOARD_SIZE).collect()
    }

    pub fn value(&self) -> int {
        if self.has_winner() {
            (self.remaining_moves().len() + 1) as int
        } else {
            0
        }
    }

    pub fn is_finished(&self) -> bool {
        self.has_winner() || self.has_draw()
    }

    pub fn has_draw(&self) -> bool {
         self.remaining_moves().is_empty() && !self.has_winner()
    }
}

fn keep_moves(slot: &Slot) -> Option<uint> {
    match *slot {
        Slot::Move(m) => Some(m),
        _ => None,
    }
}

fn all_lines(board: &[Slot]) ->Vec<line::Line> {
    let mut lines:Vec<line::Line> = Vec::new();
    lines.push_all(&rows(board));
    lines.push_all(&columns(board));
    lines.push_all(&diagonals(board));
    lines
}

fn rows(board: &[Slot]) -> [line::Line, ..3]  {
    [of(board, 0, 1 ,2),
     of(board, 3, 4 ,5),
     of(board, 6, 7 ,8)]
}

fn columns(board: &[Slot]) -> [line::Line, ..3]  {
    [of(board, 0, 3 ,6),
     of(board, 1, 4 ,7),
     of(board, 2, 5 ,8)]
}

fn diagonals(board: &[Slot]) -> [line::Line, ..2]  {
    [of(board, 0, 4 ,8),
     of(board, 6, 4 ,2)]
}

fn of(board: &[Slot], first: uint, second: uint, third: uint) -> line::Line {
    line::from_slots(board[first], board[second], board[third])
}

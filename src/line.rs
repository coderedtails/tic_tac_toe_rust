#[deriving(Clone)]
pub struct Line(Player, Player, Player);

#[deriving(Clone)]
#[deriving(PartialEq)]
pub enum Player {
    X,
    O,
    Empty,
}

pub enum WinnerResult {
    Winner(Player),
    NoWinner,
}

pub trait Winnable {
    fn winner(&self) -> WinnerResult;
}

impl Winnable for Line {
    fn winner(&self) -> WinnerResult {
        match *self {
            Line(Player::X, Player::X, Player::X) => WinnerResult::Winner(Player::X),
            Line(Player::O, Player::O, Player::O) => WinnerResult::Winner(Player::O),
            Line(_,_,_) => WinnerResult::NoWinner,
        }
    }
}

pub fn is_winner_of_line(line: Line, player: Player) -> bool {
    match line.winner() {
        WinnerResult::Winner(n) if n == player => true,
        _ => false,
    }
}

pub fn has_no_winner(line: Line) -> bool {
    match line.winner() {
        WinnerResult::NoWinner => true,
        _ => false,
    }
}

pub fn empty() -> Line {
    new(Player::Empty, Player::Empty, Player::Empty)
}

pub fn new(first: Player, second: Player, third: Player) -> Line {
    Line(first, second, third)
}

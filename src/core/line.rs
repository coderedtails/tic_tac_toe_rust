use core::marker::Marker;

#[deriving(Clone)]
pub struct Line(Marker, Marker, Marker);

pub enum WinnerResult {
    Winner(Marker),
    NoWinner,
}

pub trait Winnable {
    fn winner(&self) -> WinnerResult;
    fn is_winner(&self, player: &Marker) -> bool;
    fn no_winner(&self) -> bool;
}

pub fn empty() -> Line {
    new(Marker::Empty, Marker::Empty, Marker::Empty)
}

pub fn new(first: Marker, second: Marker, third: Marker) -> Line {
    Line(first, second, third)
}


impl Winnable for Line {
    fn winner(&self) -> WinnerResult {
        match *self {
            Line(Marker::X, Marker::X, Marker::X) => WinnerResult::Winner(Marker::X),
            Line(Marker::O, Marker::O, Marker::O) => WinnerResult::Winner(Marker::O),
            Line(_,_,_) => WinnerResult::NoWinner,
        }
    }

    fn is_winner(&self, player: &Marker) -> bool {
        match self.winner() {
            WinnerResult::Winner(n) if n == *player => true,
            _ => false,
        }
    }

    fn no_winner(&self) -> bool {
        match self.winner() {
            WinnerResult::NoWinner => true,
            _ => false,
        }
    }
}
use core::marker::Marker;

#[deriving(Clone)]
pub struct Line(Marker, Marker, Marker);

pub enum WinnerResult {
    Winner(Marker),
    NoWinner,
}

pub fn empty() -> Line {
    new(Marker::Empty, Marker::Empty, Marker::Empty)
}

pub fn new(first: Marker, second: Marker, third: Marker) -> Line {
    Line(first, second, third)
}


impl Line {
    pub fn winner(&self) -> WinnerResult {
        match *self {
            Line(Marker::X, Marker::X, Marker::X) => WinnerResult::Winner(Marker::X),
            Line(Marker::O, Marker::O, Marker::O) => WinnerResult::Winner(Marker::O),
            Line(_,_,_) => WinnerResult::NoWinner,
        }
    }

    pub fn is_winner(&self, player: &Marker) -> bool {
        match self.winner() {
            WinnerResult::Winner(n) if n == *player => true,
            _ => false,
        }
    }

    pub fn no_winner(&self) -> bool {
        match self.winner() {
            WinnerResult::NoWinner => true,
            _ => false,
        }
    }
}

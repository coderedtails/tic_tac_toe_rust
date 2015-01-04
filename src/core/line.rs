use core::marker::Marker;
use core::slot::Slot;

#[derive(Clone)]
pub struct Line(pub Option<Marker>, pub Option<Marker>, pub Option<Marker>);

pub enum WinnerResult {
    Winner(Marker),
    NoWinner,
}

pub fn from_slots(first: Slot, second: Slot, third: Slot) -> Line {
    Line(convert(first), convert(second), convert(third))
}

fn convert(slot: Slot) -> Option<Marker> {
    match slot {
        Slot::Placed(m) => Some(m),
        Slot::Move(_) => None,
    }
}

impl Line {
    pub fn winner(&self) -> WinnerResult {
        match *self {
            Line(Some(a), Some(b),Some(c)) if a == b && a == c => WinnerResult::Winner(a),
            Line(_,_,_) => WinnerResult::NoWinner,
        }
    }

    pub fn is_winner(&self, player: &Marker) -> bool {
        match self.winner() {
            WinnerResult::Winner(ref n) if n == player => true,
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

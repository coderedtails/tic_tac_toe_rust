#[deriving(Clone)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub enum Marker {
    X,
    O,
    Empty,
}

impl Marker {
    pub fn as_string(&self, idx: uint) -> String {
        match *self {
            Marker::Empty => format!("{}", idx).to_string(),
            _ => self.to_string(),
        }
    }

    pub fn opponent(&self) -> Marker {
        match *self {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
            Marker::Empty => Marker::Empty,
        }
    }
}

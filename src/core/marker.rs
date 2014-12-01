#[deriving(Clone)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub enum Marker {
    X,
    O,
    Empty,
}

impl Marker {
    pub fn to_string(&self, idx: uint) -> String {
        match *self {
            Marker::X => "X".to_string(),
            Marker::O => "O".to_string(),
            Marker::Empty => format!("{}", idx).to_string(),
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

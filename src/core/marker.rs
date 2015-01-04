#[derive(Show, Clone, PartialEq)]
pub enum Marker {
    X,
    O,
}

impl Marker {
    pub fn opponent(&self) -> Marker {
        match *self {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
        }
    }
}

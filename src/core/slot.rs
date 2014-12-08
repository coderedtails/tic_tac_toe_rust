use core::marker::Marker;

#[deriving(Show, Clone, PartialEq)]
pub enum Slot {
    Placed(Marker),
    Move(uint)
}

impl Slot {
    pub fn printable(self) -> String {
        match self {
            Slot::Placed(p) => p.to_string(),
            Slot::Move(m) =>  m.to_string(),
        }
    }
}

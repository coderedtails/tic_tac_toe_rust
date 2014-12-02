use players::Player;
use std::cell::RefCell;

pub struct GameMode<T:Player, R: Player> {
    pub first: T,
    pub second: R,
    counter: RefCell<uint>,
}

pub fn new<T:Player, R: Player>(first: T, second: R) -> GameMode<T,R> {
    GameMode { first: first, second: second, counter: RefCell::new(0u) }
}

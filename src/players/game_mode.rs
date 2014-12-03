use players::Player;
use std::cell::Cell;

pub struct GameMode<T:Player, R: Player> {
    pub first: T,
    pub second: R,
    counter: Cell<uint>,
}

impl<T:Player, R:Player> GameMode<T,R> {
   pub fn next(&self) -> &Player {
       let val: uint = self.counter.get();
       let player: &Player = if val % 2 == 0 {
           &self.first
       } else {
           &self.second
        };

        self.counter.set(val + 1);
        player
   }
}

impl<T: Player, R: Player> GameMode<T,R> {
    pub fn opponents_line(&self) -> String {
        format!("{} vs. {}", self.first.player_type(), self.second.player_type())
    }
}

pub fn new<T:Player, R: Player>(first: T, second: R) -> GameMode<T,R> {
    GameMode { first: first, second: second, counter: Cell::new(0u) }
}

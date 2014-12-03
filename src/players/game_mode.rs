use players::Player;
use std::cell::Cell;

pub struct GameMode<'a>{
    pub first:  Box<Player + 'a>,
    pub second: Box<Player + 'a>,
    pub counter: Cell<uint>,
}

impl<'a> GameMode<'a> {
   pub fn next(&self) -> &Player {
       let val: uint = self.counter.get();
       let player: &Player = if val % 2 == 0 {
           &*self.first
       } else {
           &*self.second
        };

        self.counter.set(val + 1);
        player
   }
}

impl<'a> GameMode<'a> {
    pub fn opponents_line(&self) -> String {
        format!("{} vs. {}", self.first.player_type(), self.second.player_type())
    }
}

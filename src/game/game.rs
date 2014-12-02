use io::IO;
use io::display::Display;
use players::game_mode::GameMode;
use players::Player;

pub struct Game<P> {
    pub display: Display<P>,
}

impl <P: IO>Game<P>{
    pub fn play<T: Player, R: Player>(&self, mode: GameMode<T,R>) {
    }
}

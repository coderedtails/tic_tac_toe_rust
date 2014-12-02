use players::Player;

pub struct GameMode<T:Player, R: Player> {
    pub first: T,
    pub second: R,
}

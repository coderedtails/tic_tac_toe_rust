pub mod line {
    pub enum Player {
        X,
        O,
        None
    }
    pub struct Line(Player, Player, Player);

    pub fn new(first: Player, second: Player, third: Player) -> Line {
        Line(first, second, third)
    }
}


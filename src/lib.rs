pub mod line {
    pub enum Player {
        X,
        O,
        None
    }
    pub struct Line(Player, Player, Player);

    pub fn has_winner(line: Line) -> bool {
        match line {
            Line(Player::X, Player::X, Player::X) => true,
            Line(Player::O, Player::O, Player::O) => true,
            Line(_,_,_)         => false
        }
    }

    pub fn empty() -> Line {
        new(Player::None, Player::None, Player::None)
    }

    pub fn new(first: Player, second: Player, third: Player) -> Line {
        Line(first, second, third)
    }
}


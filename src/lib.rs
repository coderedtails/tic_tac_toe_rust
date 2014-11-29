pub mod line {
    pub struct Line(Player, Player, Player);

    #[deriving(PartialEq)]
    pub enum Player {
        X,
        O,
        Empty,
    }

    pub enum WinnerResult {
        Winner(Player),
        NoWinner,
    }

    pub fn has_winner(line: Line) -> WinnerResult {
        match line {
            Line(Player::X, Player::X, Player::X) => WinnerResult::Winner(Player::X),
            Line(Player::O, Player::O, Player::O) => WinnerResult::Winner(Player::O),
            Line(_,_,_)                           => WinnerResult::NoWinner,
        }
    }

    pub fn empty() -> Line {
        new(Player::Empty, Player::Empty, Player::Empty)
    }

    pub fn new(first: Player, second: Player, third: Player) -> Line {
        Line(first, second, third)
    }
}

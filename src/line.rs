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

pub fn has_winner_line(players: [Player, ..3]) -> WinnerResult {
    has_winner(new(players[0], players[1], players[2]))
}

pub fn has_winner(line: Line) -> WinnerResult {
    match line {
        Line(Player::X, Player::X, Player::X) => WinnerResult::Winner(Player::X),
        Line(Player::O, Player::O, Player::O) => WinnerResult::Winner(Player::O),
        Line(_,_,_)                           => WinnerResult::NoWinner,
    }
}

pub fn is_winner_of_line(line: Line, player: Player) -> bool {
    match has_winner(line) {
        WinnerResult::Winner(n) if n == player => true,
        _ => false,
    }
}

pub fn has_no_winner(line: Line) -> bool {
    match has_winner(line) {
        WinnerResult::NoWinner => true,
        _ => false,
    }
}

pub fn empty() -> Line {
    new(Player::Empty, Player::Empty, Player::Empty)
}

pub fn new(first: Player, second: Player, third: Player) -> Line {
    Line(first, second, third)
}

pub fn of_slice(players: &[Player]) -> Line {
    new(players[0], players[1], players[2])
}

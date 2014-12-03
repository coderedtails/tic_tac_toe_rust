use core::board::Board;

pub mod ai;
pub mod human;
pub mod game_mode;
pub mod scripted_player;

pub trait Player {
    fn make_move(&self, board: Board) -> Board;
    fn player_type(&self) -> String;
}


use std::cell::RefCell;

use core::board::Board;
use core::marker::Marker;
use players::Player;

pub struct ScriptedPlayer {
    pub name: Marker,
    pub moves: RefCell<Vec<uint>>,
}

impl Player for ScriptedPlayer {
    fn make_move(&self, board: Board) -> Board {
        match self.moves.borrow_mut().pop() {
            Some(n) => board.make_move(n, &self.name),
            None => panic!("No moves left!"),
        }
    }

    fn player_type(&self) -> String {
        "Scriptable".to_string()
    }
}

pub fn new_with_moves(name: Marker, moves: Vec<uint>) -> ScriptedPlayer {
    ScriptedPlayer {  name: name,
                      moves: RefCell::new(moves) }
}

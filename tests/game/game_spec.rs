#[cfg(test)]

use tic_tac_toe::players::scripted_player;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode::GameMode;
use std::cell::Cell;

#[test]
fn runs_until_there_is_a_draw() {
    let display =  ::create_spy_display();

    let ai    = scripted_player::new_with_moves(Marker::X, vec![0u, 1,5,6,8]);
    let other = scripted_player::new_with_moves(Marker::O, vec![3u, 4,2,7]);

    let game_mode = GameMode { first: box ai, second: box other, counter: Cell::new(0u) };

    let mut game = Game { display: display};
    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "There was a draw");
}

#[test]
fn runs_until_there_is_a_winner() {
    let display =  ::create_spy_display();

    let ai    = scripted_player::new_with_moves(Marker::X, vec![0u, 1,2]);
    let other = scripted_player::new_with_moves(Marker::O, vec![6u, 7]);

    let game_mode = GameMode { first: box ai, second: box other, counter: Cell::new(0u) };

    let mut game = Game { display: display};
    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "The winner was X");
}

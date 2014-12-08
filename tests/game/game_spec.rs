#[cfg(test)]

use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::game::game_mode;
use tic_tac_toe::game::game_runner;
use tic_tac_toe::players::scripted_player;

#[test]
fn runs_until_there_is_a_draw() {
    let display =  ::create_spy_display();
    let mut game = game_runner::new(display);

    let ai    = scripted_player::new_with_moves(Marker::X, vec![1u, 2,6,7,9]);
    let other = scripted_player::new_with_moves(Marker::O, vec![4u, 5,3,8]);
    let game_mode = game_mode::new(box ai, box other);

    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "There was a draw");
}

#[test]
fn runs_until_there_is_a_winner() {
    let display =  ::create_spy_display();
    let mut game = game_runner::new(display);

    let ai    = scripted_player::new_with_moves(Marker::X, vec![1u, 2, 3]);
    let other = scripted_player::new_with_moves(Marker::O, vec![7u, 8]);
    let game_mode = game_mode::new(box ai, box other);

    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "The winner was X");
}

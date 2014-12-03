#[cfg(test)]

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::players::scripted_player;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::game::game::Game;
use tic_tac_toe::players::game_mode;

#[test]
fn runs_until_there_is_a_draw() {
    let cli = cli_spy::new();
    let display =  Display { cli: cli };

    let ai    = scripted_player::new_with_moves(Marker::X, vec![0u, 1,5,6,8]);
    let other = scripted_player::new_with_moves(Marker::O, vec![3u, 4,2,7]);

    let game_mode = game_mode::new(ai,other);

    let mut game = Game { display: display};
    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "There was a draw");
}

#[test]
fn runs_until_there_is_a_winner() {
    let cli = cli_spy::new();
    let display =  Display { cli: cli };

    let ai    = scripted_player::new_with_moves(Marker::X, vec![0u, 1,2]);
    let other = scripted_player::new_with_moves(Marker::O, vec![6u, 7]);

    let game_mode = game_mode::new(ai,other);

    let mut game = Game { display: display};
    game.play(game_mode);
    ::assert_printed(&mut game.display.cli, "The winner was X");
}

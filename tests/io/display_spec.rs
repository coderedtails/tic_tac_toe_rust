#[cfg(test)]

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::display::Display;
use tic_tac_toe::core::board;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::game_mode;

#[test]
fn prints_an_empty_board() {
    let result  = "[1][2][3]\n[4][5][6]\n[7][8][9]";
    let mut display = ::create_spy_display();
    display.draw(board::empty());
    ::assert_printed(&mut display.cli, result);
}

#[test]
fn prints_a_non_empty_board() {
    let board = ::board_from_str("XXX---OOO");
    let result  = "[X][X][X]\n[4][5][6]\n[O][O][O]";
    let mut display = ::create_spy_display();
    display.draw(board);
    ::assert_printed(&mut display.cli, result);
}

#[test]
fn request_a_valid_move() {
    let cli_spy = cli_spy::new_with_input("1");
    let mut display = Display { cli: cli_spy, use_colour: false };
    let result = display.request_move();
    assert_eq!(result, 1);
    ::assert_printed(&mut display.cli, "Choose move");
}

#[test]
fn request_a_valid_game_mode() {
    let cli_spy = cli_spy::new_with_input("1");
    let mut display = Display { cli: cli_spy, use_colour: false };
    let result = display.request_mode();
    assert_eq!(result, 1);
    ::assert_printed(&mut display.cli, "Choose game mode:");
}

#[test]
fn requests_a_rematch_accepted() {
    let cli_spy = cli_spy::new_with_input("Y\n");
    let mut display = Display { cli: cli_spy, use_colour: false };
    let result = display.request_rematch();
    assert_eq!(result, true);
    ::assert_printed(&mut display.cli, "Want a rematch? (Y/N)");
}

#[test]
fn requests_a_rematch_denied() {
    let cli_spy = cli_spy::new_with_input("N");
    let mut display = Display { cli: cli_spy, use_colour: false };
    let result = display.request_rematch();
    assert_eq!(result, false);
    ::assert_printed(&mut display.cli, "Want a rematch? (Y/N)");
}

#[test]
fn announces_x_as_the_winner() {
    let mut display = ::create_spy_display();
    display.announce_winner(Marker::X);
    ::assert_printed(&mut display.cli, "The winner was X");
}

#[test]
fn announces_a_draw() {
    let mut display = ::create_spy_display();
    display.announce_draw();
    ::assert_printed(&mut display.cli, "There was a draw");
}

#[test]
fn prints_game_modes() {
    let mut display = ::create_spy_display();
    let game_mode = game_mode::ai_vs_ai();

    display.show_option(&game_mode, 1);
    ::assert_printed(&mut display.cli, "1: Ai vs. Ai");
}

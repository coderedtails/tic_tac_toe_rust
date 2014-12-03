#[cfg(test)]

use tic_tac_toe::io::display::Display;
use tic_tac_toe::io::cli_spy;
use tic_tac_toe::core::board::Board;
use tic_tac_toe::core::marker::Marker;

static BOARD: Board = Board{ marks: [Marker::Empty,..9]};

#[test]
fn prints_an_empty_board() {
    let result  = "[0][1][2]\n[3][4][5]\n[6][7][8]";
    let mut display = ::create_spy_display();
    display.draw(BOARD);
    ::assert_printed(&mut display.cli, result);
}

#[test]
fn prints_a_non_empty_board() {
    let board = ::board_from_str("XXX---OOO");
    let result  = "[X][X][X]\n[3][4][5]\n[O][O][O]";
    let mut display = ::create_spy_display();
    display.draw(board);
    ::assert_printed(&mut display.cli, result);
}
#[test]
fn request_a_valid_move() {
    let cli_spy = cli_spy::new_with_moves(vec!["1".to_string()]);
    let mut display = Display { cli: cli_spy, use_colour: false };
    let result = display.request_move();
    assert_eq!(result, 1);
    ::assert_printed(&mut display.cli, "Choose move");
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

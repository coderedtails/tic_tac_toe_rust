#[cfg(test)]

use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::ai::Ai;

static AI: Ai = Ai {name: Marker::X};

#[test]
fn ai_can_block_in_the_first_row() {
    let board = ::board_from_str("O-O------");
    let result = AI.best_move(board);
    assert_eq!(result,2u);
}

#[test]
fn takes_corners() {
    let board = ::board_from_str("O---O---X");

    let result = AI.best_move(board);
    let expected = vec![3u, 7];
    assert!(expected.contains(&result));
}

#[test]
fn takes_edges() {
    let board = ::board_from_str("O---X---O");
    let result = AI.best_move(board);
    let expected = vec![2u, 4, 6, 8];
    assert!(expected.contains(&result));
}

#[test]
fn takes_top_left() {
    let board = ::board_from_str("-O-OX----");
    let result = AI.best_move(board);
    let expected = vec![1u, 3, 7];
    assert!(expected.contains(&result));
}

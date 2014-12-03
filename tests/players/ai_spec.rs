#[cfg(test)]

use tic_tac_toe::core::board::Board;
use tic_tac_toe::core::marker::Marker;
use tic_tac_toe::players::ai;
use tic_tac_toe::players::Player;

#[test]
fn ai_can_block_in_the_first_row() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::O,
                                Marker::Empty, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let expected = Board { marks: [Marker::O, Marker::X, Marker::O,
                                   Marker::Empty, Marker::Empty, Marker::Empty,
                                   Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.make_move(board);
    assert_eq!(result, expected);
}

#[test]
fn ai_find_the_best_move_1() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::O,
                                Marker::Empty, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.best_move(board);
    assert_eq!(result, 1u);
}

#[test]
fn ai_find_the_best_move_2() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::O, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.best_move(board);
    assert_eq!(result, 2u);
}

#[test]
fn takes_corners() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::O, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::X] };

    let result = player.best_move(board);
    let expected = vec![2u, 6];
    assert!(expected.contains(&result));
}

#[test]
fn takes_edges() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::O, Marker::Empty, Marker::Empty,
                                Marker::Empty, Marker::X, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::O] };

    let result = player.best_move(board);
    let expected = vec![1u, 3, 5, 7];
    assert!(expected.contains(&result));
}

#[test]
fn takes_top_left() {
    let player = ai::new(Marker::X);
    let board = Board { marks: [Marker::Empty, Marker::O, Marker::Empty,
                                Marker::O, Marker::X, Marker::Empty,
                                Marker::Empty, Marker::Empty, Marker::Empty] };

    let result = player.best_move(board);
    let expected = vec![0u, 2, 6];
    assert!(expected.contains(&result));
}

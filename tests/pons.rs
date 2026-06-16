//! Integration tests from Pascal Pons blog

mod common;
use common::*;

use connect4_engine::{DefaultBoard as Board, Solver};

fn run_test_case(path: &str) {
    let cases = load_test_suite(path).unwrap();

    let mut solver = Solver::new();

    for case in cases {
        let mut board = Board::new();
        board.try_play_string(&case.opening_sequence).unwrap();

        let result = solver.evaluate(&board);

        assert_eq!(result, case.expected_evaluation);

        // TODO: Should tests clear tt?
        solver.clear();
    }
}

#[test]
fn test_easy_end_game_boards() {
    run_test_case("./data/tests/easy_end.test");
}

#[test]
fn test_easy_mid_game_boards() {
    run_test_case("./data/tests/easy_middle.test");
}

#[test]
fn test_easy_early_game_boards() {
    run_test_case("./data/tests/easy_early.test");
}

#[ignore]
#[test]
fn test_medium_mid_game_boards() {
    run_test_case("./data/tests/medium_middle.test");
}

#[ignore]
#[test]
fn test_medium_early_game_boards() {
    run_test_case("./data/tests/medium_early.test");
}

#[ignore]
#[test]
fn test_hard_early_game_boards() {
    run_test_case("./data/tests/hard_early.test");
}

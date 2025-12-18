//! Integration tests/benchmarks from Pascal Pons blog

use connect4_engine::{board::Board, solver::Solver};

fn run_test_case(path: &str) {
    let file =
        std::fs::read_to_string(&path).expect(&format!("could not read test data from `{path}`"));

    let mut solver = Solver::<7, 6>::new();

    for (r, line) in file.lines().enumerate() {
        let mut parts = line.split_ascii_whitespace();
        let sequence = parts
            .next()
            .expect("invalid test format; expected move sequence");
        let expected = parts
            .next()
            .expect("invalid test format; expected expected result")
            .parse()
            .expect("invalid test data; unable to parse expected result");

        let mut board = Board::new();

        let iter = sequence.chars().map(|ch| {
            ch.try_into().expect(&format!(
                "invalid test data; unable to play column `{ch}` in {path:?}:{r}"
            ))
        });

        board.play_sequence(iter).expect(&format!(
            "invalid move sequence `{sequence}` in {path:?}:{r}"
        ));

        let result = solver.evaluate(&board);

        assert_eq!(result, expected);

        // solver.clear();
    }
}

#[test]
fn easy_end_game_boards() {
    run_test_case("./tests/data/end_easy.test");
}

#[test]
fn easy_mid_game_boards() {
    run_test_case("./tests/data/middle_easy.test");
}

#[test]
fn easy_early_game_boards() {
    run_test_case("./tests/data/early_easy.test");
}

#[test]
#[ignore]
fn medium_mid_game_boards() {
    run_test_case("./tests/data/middle_medium.test");
}

#[test]
#[ignore]
fn medium_early_game_boards() {
    run_test_case("./tests/data/early_medium.test");
}

#[test]
#[ignore]
fn hard_early_game_boards() {
    run_test_case("./tests/data/early_hard.test");
}

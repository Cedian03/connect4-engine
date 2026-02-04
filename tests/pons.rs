//! Integration tests/benchmarks from Pascal Pons blog

use connect4_engine::{DefaultBoard, Solver};

fn run_test_case(path: &str) {
    let file =
        std::fs::read_to_string(&path).expect(&format!("could not read test data from `{path}`"));

    let mut solver = Solver::new();

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

        let mut board = DefaultBoard::new();

        board.try_play_string(sequence).expect(&format!(
            "invalid move sequence `{sequence}` in {path:?}:{r}"
        ));

        let result = solver.evaluate(&board);

        assert_eq!(result, expected);

        // solver.clear();
    }
}

#[test]
fn test_easy_end_game_boards() {
    run_test_case("./tests/data/end_easy.test");
}

#[test]
fn test_easy_mid_game_boards() {
    run_test_case("./tests/data/middle_easy.test");
}

#[test]
fn test_easy_early_game_boards() {
    run_test_case("./tests/data/early_easy.test");
}

#[test]
#[ignore]
fn test_medium_mid_game_boards() {
    run_test_case("./tests/data/middle_medium.test");
}

#[test]
#[ignore]
fn test_medium_early_game_boards() {
    run_test_case("./tests/data/early_medium.test");
}

#[test]
#[ignore]
fn test_hard_early_game_boards() {
    run_test_case("./tests/data/early_hard.test");
}

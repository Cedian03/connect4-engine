#[path = "../../tests/common/mod.rs"]
mod common;

use std::time::{Duration, Instant};

use connect4_engine::{DefaultBoard as Board, Solver};

use common::*;

const BENCHMARKS: &[&str] = &[
    "easy_end",
    "easy_middle",
    "easy_early",
    "medium_middle",
    "medium_early",
    "hard_early",
];

fn main() {
    println!("| suite         | avg time     |  avg pos     | avg pos/sec  |");
    println!("|---------------+--------------+--------------+--------------|");

    let mut solver = Solver::default();

    for suite in BENCHMARKS {
        let path = format!(
            "{}/../data/tests/{}.test",
            env!("CARGO_MANIFEST_DIR"),
            suite
        );

        let cases = load_test_suite(path).unwrap();

        let mut total_time = Duration::ZERO;
        let mut total_searched = 0;

        let n = cases.len();

        for case in cases {
            let mut board = Board::default();
            board.try_play_string(&case.opening_sequence).unwrap();

            let start = Instant::now();
            let evaluation = solver.evaluate(&board);
            let elapsed = start.elapsed();

            assert_eq!(evaluation, case.expected_evaluation);

            total_time += elapsed;
            total_searched += solver.searched();

            solver.clear();
        }

        let avg_time = total_time / n as u32;
        let avg_searched = total_searched / n as u64;
        let avg_searched_per_sec = avg_searched as f32 / avg_time.as_secs_f32();

        println!(
            "| {suite:>13} | {avg_time:>12.1?} | {avg_searched:>12} | {avg_searched_per_sec:>12} |"
        )
    }
}

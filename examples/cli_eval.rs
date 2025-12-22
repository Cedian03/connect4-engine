use std::time::Instant;

use clap::Parser;

use connect4_engine::{util::char_to_col, Board, OpeningBook, Solver};

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    sequence: Option<String>,
    #[arg(long)]
    no_book: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut board: Board<7, 6> = Board::new();
    if let Some(s) = cli.sequence {
        for (i, ch) in s.char_indices() {
            board.play_col(
                char_to_col(ch).expect(&format!("invalid column `{ch}` at index {i} of sequence")),
            );
        }
    }

    let mut solver = Solver::new();
    if !cli.no_book {
        solver = solver.with_book(OpeningBook::open("./.book").expect("missplaced opening book"))
    }

    let start = Instant::now();
    let evaluation = solver.evaluate(&board);
    let duration = start.elapsed();

    println!(
        "Searched {} nodes in {:.3?} ({} nodes/s) to get an evaluatiuon of {}!",
        solver.searched(),
        duration,
        solver
            .searched()
            .checked_div(duration.as_millis() as u64)
            .map_or("INFINITE".to_string(), |x| (x * 1000).to_string()),
        evaluation,
    );
}

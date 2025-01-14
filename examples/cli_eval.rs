use std::time::Instant;

use clap::Parser;

use connect4_engine::{OpeningBook, Position, Solver};

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    sequence: Option<String>,
    #[arg(long)]
    no_book: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut position = Position::standard();
    if let Some(s) = cli.sequence {
        for (i, ch) in s.char_indices() {
            position.play_col(
                char_to_col(&position, ch)
                    .expect(&format!("invalid column `{ch}` at index {i} of sequence")),
            );
        }
    }

    let mut solver = (!cli.no_book)
        .then(|| Solver::with_book(OpeningBook::open("./.book").unwrap()))
        .unwrap_or_default();

    let start = Instant::now();
    let evaluation = solver.evaluate(&position);
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

fn char_to_col(position: &Position, ch: char) -> Option<usize> {
    (ch as usize)
        .checked_sub('A' as usize)
        .filter(|col| *col < position.width())
}
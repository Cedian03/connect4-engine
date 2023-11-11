use std::time::Instant;

use clap::Parser;
use thousands::Separable;

use connect4_engine::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short = 's', long = "sequence")]
    sequence: Option<String>,
    #[arg(short = 'o', long = "opening-book")]
    opening_book: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut position = Position::default();
    let mut solver = Solver::new();

    if let Some(seq) = args.sequence {
        position.play_seq(seq);
    }

    if let Some(path) = args.opening_book {
        solver.load_book(path)
    }

    println!("Solving position:\n{}", &position);

    let start = Instant::now();
    let evaluation = solver.evaluate(&position);
    let duration = start.elapsed();

    let searched = solver.node_count;
    let millis = duration.as_millis();

    println!(
        "Searched {} nodes in {:.3?} ({} nodes/s) to get an evaluatiuon of {}!",
        searched.separate_with_spaces(),
        duration,
        searched.checked_div(millis as u64)
            .map_or("INFINITE".to_string(), |x| (x * 1000).separate_with_spaces()),
        evaluation,
    );
}

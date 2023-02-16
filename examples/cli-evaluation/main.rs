use connect4_engine::Position;
use connect4_engine::Solver;

use std::time::Instant;

use clap::Parser;
use thousands::Separable; 

#[derive(Parser)]
struct Args {
    #[arg(short = 's', long = "sequence")]
    sequence: Option<String>,
    #[arg(short = 'o', long = "no-opening-book")]
    no_opening_book: bool,
}

fn main() {
    let mut position = Position::new();
    let mut solver = Solver::new();

    let args = Args::parse();
    if let Some(sequence) = args.sequence {
        position.play_sequence(&sequence);
    }

    if !args.no_opening_book {
        solver.load_book(".book");
    }

    let start = Instant::now(); 
    let evaluation = solver.evaluate(&position);
    let duration = start.elapsed(); 

    println!(
        "Searched {} nodes in {:.3?} to get an evaluatiuon of {}!",
        solver.get_node_count().separate_with_underscores(),
        duration,
        evaluation,
    );
}

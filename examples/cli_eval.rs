use std::time::Instant;

use clap::Parser;

use connect4_engine::{DefaultBoard, OpeningBook, Solver};

const OPENING_BOOK_PATH: &str = "./.book";

#[derive(Parser)]
struct Cli {
    opening: Option<String>,
    #[arg(long)]
    no_book: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut board = DefaultBoard::new();
    if let Some(opening) = cli.opening {
        board.try_play_string(&opening).expect("invalid opening");
    }

    let mut solver = Solver::new();
    if !cli.no_book {
        solver = solver.with_book(OpeningBook::open(OPENING_BOOK_PATH).expect("invalid book"))
    }

    println!("Evaluating the following position:");
    println!("{}", board);

    let start = Instant::now();
    let evaluation = solver.evaluate(&board);
    let duration = start.elapsed();

    print!(
        "Searched {} nodes in {:.3?} ({} nodes/s) to get an evaluation of {}!",
        solver.searched(),
        duration,
        solver
            .searched()
            .checked_div(duration.as_millis() as u64)
            .map_or("INFINITE".to_owned(), |x| (x * 1000).to_string()),
        evaluation,
    );

    if evaluation != 0 {
        let mut winning = board.disk_to_play();
        if evaluation < 0 {
            winning = !winning;
        }

        println!(" That is winning for {}.", winning)
    } else {
        println!(" That is a draw.")
    }
}

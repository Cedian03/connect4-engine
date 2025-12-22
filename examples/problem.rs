use std::io::{self, Write};

use clap::Parser;

use connect4_engine::{util::char_to_col, Board, Col};

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    opening: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut board = Board::<7, 6>::new();
    if let Some(opening) = cli.opening {
        let sequence = opening
            .chars()
            .map(|ch| char_to_col(ch).expect("invalid column in opening"));

        board.play_sequence(sequence).expect("invalid opening");
    }

    let reset = board.clone();

    'outer: loop {
        println!("{}", board);

        print!("Enter a column to play (A-G): ");
        let _ = io::stdout().flush();

        let mut col = 123;

        loop {
            let mut buf = String::new();
            let _ = io::stdin().read_line(&mut buf);

            if buf == "R" {
                board = reset;
                break;
            }

            if let Some(ch) = buf.chars().next() {
                if let Some(col) = char_to_col(ch) {
                    if board.can_play(col) {
                        break;
                    }
                }
            }

            println!("Try again: ");
            let _ = io::stdout().flush();
        }
    }
}

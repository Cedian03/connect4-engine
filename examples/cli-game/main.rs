use std::io;

use clap::Parser;

use connect4_engine::prelude::*;
use connect4_engine::util::{col_to_char, char_to_col};

#[derive(Parser)]
struct Args {
    #[arg(short = 'x', long = "x-as-engine")]
    x_is_engine: bool,
    #[arg(short = 'o', long = "o-as-engine")]
    o_as_engine: bool,
}

fn handle_player_input() -> usize {
    let mut input = String::new();
    loop {
        println!("Enter your move (A-{}):", col_to_char(Position::WIDTH - 1).unwrap());

        let _ = io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Couldn't read input, try again");

        if let Some(ch) = input.chars().next() {
            if let Ok(col) = char_to_col(ch) {
                return col
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut position = Position::new();
    let mut solver = Solver::new();
    solver.load_book("./.book")?;

    let mut playing = true;
    while playing {
        println!("{}", position);

        let col = {
            if (position.half_turn() % 2 == 0 && !args.x_is_engine)
                || (position.half_turn() % 2 == 1 && !args.o_as_engine) 
            {
                handle_player_input()
            } else {
                solver.analyze(&position)
                    .into_iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(i, _)| i)
                    .unwrap()
            }
        };

        playing = !(position.is_winning_move(col) || position.half_turn() == 40);
        position.play_col(col);
    }

    println!("{}", position);
    Ok(())
}

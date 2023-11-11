use std::io;

use clap::Parser;

use connect4_engine::prelude::*;
use connect4_engine::util::{col_to_char, char_to_col};

#[derive(Parser)]
struct Args {
    #[arg(short = 'x', long = "x-as-engine")]
    x_is_engine: bool,
    #[arg(short = 'o', long = "o-as-human")]
    o_as_human: bool,
}

fn handle_player_input() -> usize {
    let mut input = String::new();
    loop {
        println!("Enter your move (A-{}):", col_to_char(Position::WIDTH - 1).unwrap());

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(ch) = input.chars().next() {
            if let Some(col) = char_to_col(ch) {
                return col
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut position = Position::new();
    let mut solver = Solver::new();
    solver.load_book(".book");

    let mut playing = true;
    while playing {
        println!("{}", position);

        let col = {
            if (position.half_turn() % 2 == 0 && !args.x_is_engine)
                || (position.half_turn() % 2 == 1 && args.o_as_human) 
            {
                handle_player_input()
            } else {
                solver.anazlyse(&position)
                    .into_iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(i, _)| i)
                    .unwrap()
            }
        };

        playing = !position.is_winning_move(col);
        position.play_col(col);
    }
    println!("{}", position);
}
use connect4_engine::GameHandler;
use connect4_engine::State;

use std::io;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'x', long = "x-as-engine")]
    x_is_engine: bool,
    #[arg(short = 'o', long = "o-as-human")]
    o_as_human: bool,
}

fn main() {
    let args = Args::parse();
    let mut game_handler = GameHandler::new();

    while game_handler.state == State::InProgress {
        println!("{}", game_handler);

        let col;
        if (game_handler.turn % 2 == 1 && !args.x_is_engine)
            || (game_handler.turn % 2 == 0 && args.o_as_human)
        {
            loop {
                println!("Enter your move (A-G):");
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input = input.trim();
                col = match input {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    "D" => 3,
                    "E" => 4,
                    "F" => 5,
                    "G" => 6,
                    _ => {
                        println!("Invalid input: {} (out of range)", input);
                        continue;
                    }
                };

                break;
            }
        } else {
            col = game_handler.optimal_col();
        }
        game_handler.play(col);
    }
    println!("{}", game_handler);
}

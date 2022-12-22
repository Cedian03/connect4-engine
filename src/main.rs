mod solver;
mod position;
mod move_sorter;
mod opening_book;
mod transposition_table;

use solver::Solver;
use position::Position;
use position::GameState;

use clap::Parser;
use console::Term;

#[derive(Parser)]
struct Args {
    #[arg(short = 'x', long = "x_is_human")]
    x_is_human: bool,
    #[arg(short = 'o', long = "o_is_human")]
    o_is_human: bool,
}

fn human_play(position: &mut Position) {
    let term = Term::stdout();
    let mut input = String::new();

    loop {
        term.write_line("Enter your move (1-7):");
        input = term.read_line().unwrap();

        if let Ok(col) = input.trim().parse::<i32>() {
            if col >= 1 && col <= 7 {
                position.play_col(col - 1);
                break;
            } else {
                println!("Invalid input: {} (out of range)", input);
            }
        } else {
            println!("Invalid input: {} (not a number)", input);
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut position = Position::new(); 
    let mut solver = Solver::new();

    solver.load_book(".book");

    while position.game_state == GameState::InProgress {
        println!("{}", position); 
        println!("{:?}", solver.analyze(&position, true));

        if position.nb_moves() % 2 == 0 {
            if args.x_is_human {
                human_play(&mut position)
            } else {
                solver.play(&mut position)
            }
        } else {
            if args.o_is_human {
                human_play(&mut position)
            } else {
                solver.play(&mut position)
            }
        }
    }
    println!("{:?}", position.game_state);
    println!("{}", position);
}


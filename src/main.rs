mod move_sorter;
mod opening_book;
mod position;
mod solver;
mod transposition_table;

use position::Position;
use position::State;
use solver::Solver;

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
        term.write_line("Enter your move (A-G):");
        input = term.read_line().unwrap();
        println!();

        let col = input.trim().chars().next().unwrap() as i32 - 65;

        if col >= 0 && col <= 6 {
            position.play_col(col);
            break;
        } else {
            println!("Invalid input: {} (out of range)", input);
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut position = Position::new();
    let mut solver = Solver::new();

    solver.load_book(".book");

    while position.get_state() == State::InProgress {
        println!("{}", position);
        println!("Analysis: {:?}\n", solver.analyze(&position, true));

        if position.nb_moves() % 2 == 0 {
            if args.x_is_human {
                human_play(&mut position);
            } else {
                solver.play(&mut position);
            }
        } else {
            if args.o_is_human {
                human_play(&mut position);
            } else {
                solver.play(&mut position);
            }
        }
    }

    println!("{}", position);
    println!("{:?}", position.get_state())
}

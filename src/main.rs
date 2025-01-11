use clap::{Parser, Subcommand};

use error::{Error, Result};
use position::Position;
use solver::Solver;

mod commands;
mod error;
mod magic;
mod position;
mod solver;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Eval {
        #[clap(default_value_t = String::new())]
        seq: String,
    },
    Play {
        #[arg(short)]
        x: bool,
        #[arg(short)]
        o: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Eval { seq } => commands::eval(&seq),
        Commands::Play { x, o } => commands::play(x, o),
    }
}

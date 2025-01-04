use std::path::PathBuf;

use clap::{Parser, Subcommand};

use error::{Error, Result};
use position::{BitMask, Position};
use solver::Solver;

mod commands;
mod error;
mod magic;
mod position;
mod solver;
mod util;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    book: Option<PathBuf>,
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
        Commands::Eval { seq } => commands::eval(&seq, cli.book),
        Commands::Play { x, o } => commands::play(x, o, cli.book),
    }
}

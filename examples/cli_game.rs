use std::io::{self, Write};

use clap::{Parser, ValueEnum};

use connect4_engine::{
    util::{char_to_col, col_to_char},
    DefaultBoard, Disk, OpeningBook, Solver,
};

const CLEAR: &str = "\x1bc";
const OPENING_BOOK_PATH: &str = "./.book";

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[arg(short = 'x', long, default_value = "human")]
    player_x: PlayerType,
    #[arg(short = 'o', long, default_value = "engine")]
    player_o: PlayerType,
    #[arg(long)]
    opening: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerType {
    Human,
    Engine,
}

fn main() {
    let cli = Cli::parse();

    let mut board = DefaultBoard::new();
    if let Some(opening) = cli.opening {
        board.try_play_string(&opening).expect("invalid opening")
    }

    let mut solver = Solver::new();
    if cli.player_x == PlayerType::Engine || cli.player_o == PlayerType::Engine {
        solver = solver.with_book(OpeningBook::open(OPENING_BOOK_PATH).expect("invalid book"))
    }

    let mut winner = None;
    let mut game_string = String::new();

    for _ in 0..(board.width() * board.height()) {
        let disk = board.disk_to_play();

        println!("{}\nA B C D E F G - {} to play", board, disk);

        let col = match disk {
            Disk::X => match cli.player_x {
                PlayerType::Human => human(&board),
                PlayerType::Engine => engine(&mut solver, &board),
            },
            Disk::O => match cli.player_o {
                PlayerType::Human => human(&board),
                PlayerType::Engine => engine(&mut solver, &board),
            },
        };

        game_string.push(col_to_char(col).unwrap());
        if let (_, true) = board.play(col).expect("invalid column") {
            winner = Some(disk);
            break;
        }

        println!("{}", CLEAR);
    }

    if let Some(disk) = winner {
        println!("{}\nA B C D E F G - {} won!", board, disk);
    } else {
        println!("{}\nA B C D E F G - It's a draw", board);
    }

    println!("\nFull game string: `{}`", game_string)
}

fn human(board: &DefaultBoard) -> usize {
    loop {
        print!("\nEnter a column to play (A-G): ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        let trimmed = buf.trim();
        if trimmed.is_empty() {
            println!("Please enter a column letter. Try again.");
            continue;
        }

        let ch = trimmed.chars().next().unwrap();

        if let Some(col) = char_to_col(ch) {
            if col >= board.width() {
                println!(
                    "Column {} out of range! Valid columns: A-G.",
                    ch.to_uppercase(),
                );
            } else if !board.can_play_column(col) {
                println!("Column {} is full! Try another.", ch.to_uppercase());
            } else {
                return col;
            }
        } else {
            println!("Invalid column '{}'! Use letters A-G.", ch);
        }
    }
}

fn engine(solver: &mut Solver<7, 6>, board: &DefaultBoard) -> usize {
    let col = solver
        .analyze(board)
        .into_iter()
        .enumerate()
        .filter_map(|(c, x)| x.map(|s| (c, s)))
        .max_by_key(|(_, s)| *s)
        .map(|(c, _)| c)
        .expect("engine found no valid column to play");

    col
}

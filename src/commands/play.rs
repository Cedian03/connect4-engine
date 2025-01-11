use std::path::PathBuf;

use crate::{Position, Result, Solver};

pub fn play(engine_as_x: bool, engine_as_o: bool, book: Option<PathBuf>) -> Result<()> {
    let mut position = Position::default();
    let mut solver = Solver::new();

    if let Some(book) = book {
        solver.open(book).unwrap();
    }

    let mut playing = true;
    while playing {
        let col = {
            if (position.half_turn() % 2 == 0 && !engine_as_x)
                || (position.half_turn() % 2 == 1 && !engine_as_o)
            {
                let col;
                let mut input = String::new();
                loop {
                    println!(
                        "Enter your move (A-{}):",
                        ('A' as u8 + position.width() as u8) as char
                    );

                    let _ = std::io::stdin()
                        .read_line(&mut input)
                        .map_err(|_| "Couldn't read input, try again");

                    if let Some(ch) = input.chars().next() {
                        if let Some(c) = (ch as usize)
                            .checked_sub('A' as usize)
                            .filter(|x| *x < position.width())
                        {
                            col = c;
                            break;
                        }
                    }
                }

                col
            } else {
                solver
                    .analyze(&position)
                    .into_iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(i, _)| i)
                    .unwrap()
            }
        };

        playing = !(position.is_winning_col(col) || position.half_turn() == 40);
        position.play_col(col);
    }

    println!("Well played!");

    Ok(())
}

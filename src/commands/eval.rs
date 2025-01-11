use std::{path::PathBuf, time::Instant};

use thousands::Separable;

use crate::{Position, Result, Solver};

pub fn eval(seq: &str, book: Option<PathBuf>) -> Result<()> {
    let mut position = Position::default();
    let mut solver = Solver::new();

    for ch in seq.chars() {
        position.play_col(
            (ch as usize)
                .checked_sub('A' as usize)
                .filter(|x| *x < position.width())
                .unwrap(),
        );
    }

    if let Some(book) = book {
        solver.open(book).unwrap();
    }

    println!(
        "Solving position found by playing the the sequence `{}`",
        seq
    );

    println!("The position:\n{}", position);

    let start = Instant::now();
    let evaluation = solver.evaluate(&position);
    let duration = start.elapsed();

    let searched = solver.searched();
    let millis = duration.as_millis();

    println!(
        "Searched {} nodes in {:.3?} ({} nodes/s) to get an evaluatiuon of {}!",
        searched.separate_with_spaces(),
        duration,
        searched
            .checked_div(millis as u64)
            .map_or("INFINITE".to_string(), |x| (x * 1000)
                .separate_with_spaces()),
        evaluation,
    );

    Ok(())
}
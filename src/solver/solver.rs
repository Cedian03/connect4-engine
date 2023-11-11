use std::default;
use std::path::Path;

use crate::prelude::*;

use crate::solver::MoveSorter;
use crate::solver::OpeningBook;
use crate::solver::transposition_table::*;

#[derive(Debug)]
pub struct Solver {
    column_order: [usize; Position::WIDTH],
    table: TranspositionTable,
    book: Option<OpeningBook>,
    pub node_count: u64,
}

impl Solver {
    const TABLE_SIZE: usize = 24;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_book<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.book = Some(OpeningBook::load(path)?);
        Ok(())
    }

    pub fn analyze(&mut self, position: &Position) -> [Option<i32>; Position::WIDTH] {
        let mut evals = [None; Position::WIDTH];
        for col in 0..Position::WIDTH {
            if position.can_play(col) {
                let mut new_position = position.clone();
                new_position.play_col(col);
                let eval = -self.evaluate(&new_position);
                evals[col] = Some(eval);
            }
        }

        evals
    }

    pub fn evaluate(&mut self, position: &Position) -> i32 {
        if position.can_win_next() {
            return (Position::AREA + 1 - position.half_turn()) / 2;
        }

        let mut min = -(Position::AREA - position.half_turn()) / 2;
        let mut max = (Position::AREA + 1 - position.half_turn()) / 2;

        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2
            } else if med >= 0 && max / 2 > med {
                med = max / 2
            }

            let r = self.negamax(position, med, med + 1);

            if r <= med {
                max = r
            } else {
                min = r
            }
        }

        min
    }

    fn negamax(&mut self, position: &Position, mut alpha: i32, mut beta: i32) -> i32 {
        assert!(alpha < beta);
        assert!(!position.can_win_next());

        self.node_count += 1;

        let possible = position.possible_non_losing_moves();
        if possible == 0 {
            return -(Position::AREA - position.half_turn()) / 2;
        }

        if position.half_turn() >= Position::AREA - 2 {
            return 0;
        }

        let mut min = -(Position::AREA - 2 - position.half_turn()) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max = (Position::AREA - 1 - position.half_turn()) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        if let Some(book) = &self.book {
            if let Some(val) = book.get(&position) {
                return val as i32 + Position::MIN_SCORE - 1;
            }
        }

        let key = position.key();
        if let Some(val) = self.table.get(key) {
            if val as i32 > Position::MAX_SCORE - Position::MIN_SCORE + 1 {
                min = val as i32 + 2 * Position::MIN_SCORE - Position::MAX_SCORE - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta {
                        return alpha;
                    }
                }
            } else {
                max = val as i32 + Position::MIN_SCORE - 1;
                if beta > max {
                    beta = max;
                    if alpha >= beta {
                        return beta;
                    }
                }
            }
        }

        let mut moves = MoveSorter::default();
        for i in (0..Position::WIDTH).rev() {
            let mask = possible & Position::column_mask(self.column_order[i]);
            if mask != 0 {
                moves.add(mask, position.move_score(mask));
            }
        }

        while let Some(next) = moves.get_next() {
            let mut position2 = position.clone();
            position2.play_mask(next);
            let score = -self.negamax(&position2, -beta, -alpha);

            if score >= beta {
                self.table.put(
                    key,
                    (score + Position::MAX_SCORE - 2 * Position::MIN_SCORE + 2) as i8,
                );
                return score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        self.table.put(key, (alpha - Position::MIN_SCORE + 1) as i8);
        alpha
    }

    fn column_order() -> [usize; Position::WIDTH] {
        core::array::from_fn(|i| match i % 2 {
            0 => Position::WIDTH / 2 + (i / 2),
            1 => Position::WIDTH / 2 - (i / 2 + 1),
            _ => unreachable!(),
        })
    }
}

impl default::Default for Solver {
    fn default() -> Self {
        Self {
            column_order: Solver::column_order(),
            table: TranspositionTable::new(Solver::TABLE_SIZE),
            book: None,
            node_count: 0,
        }
    }
}

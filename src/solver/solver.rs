use std::default;
use std::path::Path;

use super::{MoveSorter, OpeningBook, TranspositionTable};
use crate::{BitMask, Position, Result};

#[derive(Debug)]
pub struct Solver {
    table: TranspositionTable,
    book: Option<OpeningBook>,
    searched: u64,
}

impl Solver {
    const TABLE_SIZE: usize = 24;

    const MAX_DEPTH: i32 = (Position::WIDTH * Position::HEIGHT) as i32;
    const MIN_SCORE: i32 = -Self::MAX_DEPTH / 2 + 3;
    const MAX_SCORE: i32 = (Self::MAX_DEPTH + 1) / 2 - 3;

    pub const COLUMN_ORDER: [usize; Position::WIDTH] = Self::column_order();

    pub fn new() -> Self {
        Self::default()
    }

    pub fn searched(&self) -> u64 {
        self.searched
    }

    pub fn load_book<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.book = Some(OpeningBook::load(path)?);
        Ok(())
    }

    pub fn analyze(&mut self, position: &Position) -> [Option<i32>; Position::WIDTH] {
        let mut evals = [None; Position::WIDTH];
        for col in 0..Position::WIDTH {
            if position.can_play_col(col) {
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
            return (Self::MAX_DEPTH + 1 - position.half_turn()) / 2;
        }

        let mut min = -(Self::MAX_DEPTH - position.half_turn()) / 2;
        let mut max = (Self::MAX_DEPTH + 1 - position.half_turn()) / 2;

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

        self.searched += 1;

        let possible = position.possible_non_losing_moves();
        if possible == 0 {
            return -(Self::MAX_DEPTH - position.half_turn()) / 2;
        }

        if position.half_turn() >= Self::MAX_DEPTH - 2 {
            return 0;
        }

        let mut min = -(Self::MAX_DEPTH - 2 - position.half_turn()) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max = (Self::MAX_DEPTH - 1 - position.half_turn()) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        if let Some(book) = &self.book {
            if let Some(val) = book.get(&position) {
                return val as i32 + Self::MIN_SCORE - 1;
            }
        }

        let key = position.key();
        if let Some(val) = self.table.get(key) {
            if val as i32 > Self::MAX_SCORE - Self::MIN_SCORE + 1 {
                min = val as i32 + 2 * Self::MIN_SCORE - Self::MAX_SCORE - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta {
                        return alpha;
                    }
                }
            } else {
                max = val as i32 + Self::MIN_SCORE - 1;
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
            let mask = possible & Position::col_mask(Self::COLUMN_ORDER[i]);
            if mask != 0 {
                moves.add(mask, Self::score(position, mask));
            }
        }

        while let Some(next) = moves.get_next() {
            let mut position2 = position.clone();
            position2.play_mask(next);
            let score = -self.negamax(&position2, -beta, -alpha);

            if score >= beta {
                self.table.put(
                    key,
                    (score + Self::MAX_SCORE - 2 * Self::MIN_SCORE + 2) as i8,
                );
                return score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        self.table.put(key, (alpha - Self::MIN_SCORE + 1) as i8);
        alpha
    }

    const fn score(position: &Position, mask: BitMask) -> u32 {
        Position::compute_winning_positions(position.position() | mask, position.mask())
            .count_ones()
    }

    const fn column_order() -> [usize; Position::WIDTH] {
        let mut order = [0; Position::WIDTH];

        let mut i = 0;
        while i < Position::WIDTH {
            order[i] = match i % 2 {
                0 => Position::WIDTH / 2 + (i / 2),
                1 => Position::WIDTH / 2 - (i / 2 + 1),
                _ => unreachable!(),
            };

            i += 1;
        }

        order
    }
}

impl default::Default for Solver {
    fn default() -> Self {
        Self {
            table: TranspositionTable::new(Solver::TABLE_SIZE),
            book: None,
            searched: 0,
        }
    }
}

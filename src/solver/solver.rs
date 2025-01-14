use std::default;

use num_traits::{PrimInt, Zero};

use crate::{bit_mask, board::BitBoard, magic::*};

use super::{MoveSorter, OpeningBook, TranspositionTable};

#[derive(Debug)]
pub struct Solver<const W: usize, const H: usize> {
    table: TranspositionTable<W, H>,
    book: Option<OpeningBook<W, H>>,
    searched: u64,
}

impl<const W: usize, const H: usize> Solver<W, H> {
    const TABLE_LOG_SIZE: usize = 24;

    const MAX_DEPTH: i32 = (W * H) as i32;
    const MIN_SCORE: i32 = -Self::MAX_DEPTH / 2 + 3;
    const MAX_SCORE: i32 = (Self::MAX_DEPTH + 1) / 2 - 3;

    const COLUMN_ORDER: [usize; W] = Self::column_order();

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_book(mut self, book: OpeningBook<W, H>) -> Self {
        self.book = Some(book);
        self
    }

    pub fn searched(&self) -> u64 {
        self.searched
    }

    pub fn clear(&mut self) {
        self.table.clear()
    }

    const fn column_order() -> [usize; W] {
        let mut order = [0; W];

        let mut i = 0;
        while i < W {
            order[i] = match i % 2 {
                0 => W / 2 + (i / 2),
                1 => W / 2 - (i / 2 + 1),
                _ => unreachable!(),
            };

            i += 1;
        }

        order
    }
}

impl<const W: usize, const H: usize> Solver<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    pub fn analyze(&mut self, position: &BitBoard<W, H>) -> [Option<i32>; W] {
        let mut evals = [None; W];
        for col in 0..W {
            if position.can_play_col(col) {
                let mut new_position = position.clone();
                new_position.play_col(col);
                let eval = -self.evaluate(&new_position);
                evals[col] = Some(eval);
            }
        }

        evals
    }

    pub fn evaluate(&mut self, position: &BitBoard<W, H>) -> i32 {
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

    fn negamax(&mut self, position: &BitBoard<W, H>, mut alpha: i32, mut beta: i32) -> i32 {
        assert!(alpha < beta);
        assert!(!position.can_win_next());

        self.searched += 1;

        let possible = position.possible_non_losing_moves();
        if possible.is_zero() {
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
        for i in (0..W).rev() {
            let mask = possible & BitBoard::col_mask(Self::COLUMN_ORDER[i]);
            if !(mask.is_zero()) {
                moves.add(mask, Self::score(position, mask));
            }
        }

        for next in moves {
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

    fn score(position: &BitBoard<W, H>, mask: bit_mask!(W, H)) -> u32 {
        BitBoard::<W, H>::compute_winning_cells(position.curr | mask, position.mask).count_ones()
    }
}

impl<const W: usize, const H: usize> default::Default for Solver<W, H> {
    fn default() -> Self {
        Self {
            table: TranspositionTable::new(Self::TABLE_LOG_SIZE),
            book: None,
            searched: 0,
        }
    }
}

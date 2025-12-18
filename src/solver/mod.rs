mod move_sorter;
mod opening_book;
mod transposition_table;

use move_sorter::MoveSorter;
use transposition_table::{table_size, TranspositionTable};

pub use opening_book::OpeningBook;

use num_traits::{PrimInt, Zero};

use crate::{
    board::{BitBoard, Board},
    magic::*,
};

#[derive(Debug)]
pub struct Solver<const W: usize = 7, const H: usize = 6> {
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
    Board<W, H>: AsBitBoard,
{
    pub fn analyze(&mut self, board: &Board<W, H>) -> [Option<i32>; W] {
        let mut evals = [None; W];
        for col in 0..W {
            if board.can_play_col(col) {
                let mut new_board = board.clone();
                new_board.play_col(col);
                let eval = -self.evaluate(&new_board);
                evals[col] = Some(eval);
            }
        }

        evals
    }

    pub fn evaluate(&mut self, board: &Board<W, H>) -> i32 {
        if board.can_win_next() {
            return (Self::MAX_DEPTH + 1 - board.half_turn()) / 2;
        }

        let min = -(Self::MAX_DEPTH - board.half_turn()) / 2;
        let max = (Self::MAX_DEPTH + 1 - board.half_turn()) / 2;

        self.fun_name(board, min, max)
    }

    pub fn weak(&mut self, board: &Board<W, H>) -> i32 {
        if board.can_win_next() {
            return 1;
        }

        self.fun_name(board, -1, 1)
    }

    fn fun_name(&mut self, board: &Board<W, H>, mut min: i32, mut max: i32) -> i32 {
        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2
            } else if med >= 0 && max / 2 > med {
                med = max / 2
            }

            let r = self.negamax(board, med, med + 1);

            if r <= med {
                max = r
            } else {
                min = r
            }
        }

        min
    }

    fn negamax(&mut self, board: &Board<W, H>, mut alpha: i32, mut beta: i32) -> i32 {
        assert!(alpha < beta);
        assert!(!board.can_win_next());

        self.searched += 1;

        let possible = board.possible_non_losing_moves();
        if possible.is_zero() {
            return -(Self::MAX_DEPTH - board.half_turn()) / 2;
        }

        if board.half_turn() >= Self::MAX_DEPTH - 2 {
            return 0;
        }

        let mut min = -(Self::MAX_DEPTH - 2 - board.half_turn()) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max = (Self::MAX_DEPTH - 1 - board.half_turn()) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        if let Some(book) = &self.book {
            if let Some(val) = book.get(&board) {
                return val + Self::MIN_SCORE - 1;
            }
        }

        let key = board.key();
        if let Some(val) = self.table.get(key) {
            if val > Self::MAX_SCORE - Self::MIN_SCORE + 1 {
                min = val + 2 * Self::MIN_SCORE - Self::MAX_SCORE - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta {
                        return alpha;
                    }
                }
            } else {
                max = val + Self::MIN_SCORE - 1;
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
            if mask != 0.into() {
                moves.add(mask, Self::score(board, mask));
            }
        }

        for next in moves {
            let mut new_board = board.clone();
            new_board.play_mask(next);
            let score = -self.negamax(&new_board, -beta, -alpha);

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

    pub fn __foo(&self, board: Board<W, H>) -> String {
        // self.table
        //     .get(board.key())
        //     .or_else(|| self.book.as_ref().and_then(|b| b.get(&board)))
        //     .is_some()

        if self.table.get(board.key()).is_some() {
            "In table".to_owned()
        } else if self.book.as_ref().and_then(|b| b.get(&board)).is_some() {
            "In book".to_owned()
        } else {
            "Not cached".to_owned()
        }
    }

    fn score(position: &BitBoard<W, H>, mask: BitMask<W, H>) -> u32 {
        BitBoard::<W, H>::compute_winning_cells(position.curr | mask, position.mask).count_ones()
    }
}

impl<const W: usize, const H: usize> Default for Solver<W, H> {
    fn default() -> Self {
        Self {
            table: TranspositionTable::new(Self::TABLE_LOG_SIZE),
            book: None,
            searched: 0,
        }
    }
}

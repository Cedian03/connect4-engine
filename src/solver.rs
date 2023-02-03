use rand::seq::SliceRandom;

use crate::move_sorter::MoveSorter;
use crate::opening_book::OpeningBook;
use crate::position::Position;
use crate::transposition_table::*;

#[derive(Debug)]
pub struct NoValidMoveError;

pub struct Solver {
    node_count: u64,
    column_order: [i32; Position::WIDTH as usize],
    table: TranspositionTable<u32>,
    book: OpeningBook,
}

impl Solver {
    pub fn new() -> Self {
        let mut column_order: [i32; Position::WIDTH as usize] = [0; Position::WIDTH as usize];

        for i in 0..Position::WIDTH {
            column_order[i as usize] = Position::WIDTH / 2 - (i + 1) * (i % 2 * 2 - 1) / 2
        }

        return Solver {
            node_count: 0,
            column_order,
            table: TranspositionTable::new(Solver::TABLE_SIZE),
            book: OpeningBook::new(Position::WIDTH, Position::HEIGHT),
        };
    }

    const TABLE_SIZE: u8 = 24;

    fn negamax(&mut self, position: &Position, mut alpha: i32, mut beta: i32) -> i32 {
        assert!(alpha < beta);
        assert!(!position.can_win_next());

        self.node_count += 1;

        let possible: u64 = position.possible_non_losing_moves();
        if possible == 0 {
            return -(Position::WIDTH * Position::HEIGHT - position.nb_moves()) / 2;
        }

        if position.nb_moves() >= Position::WIDTH * Position::HEIGHT - 2 {
            return 0;
        }

        let mut min: i32 = -(Position::WIDTH * Position::HEIGHT - 2 - position.nb_moves()) / 2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max: i32 = (Position::WIDTH * Position::HEIGHT - 1 - position.nb_moves()) / 2;
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        if let Some(val) = self.book.get(&position) {
            if val == 0 {
                dbg!(val);
            } else {
                return val as i32 + Position::MIN_SCORE - 1;
            }
        }

        let key: u64 = position.key();
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

        let mut moves: MoveSorter = MoveSorter::new();
        for i in (0..Position::WIDTH).rev() {
            let m: u64 = possible & Position::column_mask(self.column_order[i as usize]);
            if m != 0 {
                moves.add(m, position.move_score(m));
            }
        }

        while let Some(next) = moves.get_next() {
            let mut p2: Position = position.clone();
            p2.play(next);
            let score: i32 = -self.negamax(&p2, -beta, -alpha);

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

    pub fn solve(&mut self, position: &Position, strong: bool) -> i32 {
        if position.can_win_next() {
            return (Position::WIDTH * Position::HEIGHT + 1 - position.nb_moves()) / 2;
        }

        let mut min = -(Position::WIDTH * Position::HEIGHT - position.nb_moves()) / 2;
        let mut max = (Position::WIDTH * Position::HEIGHT + 1 - position.nb_moves()) / 2;

        if !strong {
            min = -1;
            max = 1;
        }

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

    pub fn analyze(&mut self, position: &Position, strong: bool) -> Vec<Option<i32>> {
        let mut scores: Vec<Option<i32>> = vec![None; Position::WIDTH as usize];
        for col in 0..Position::WIDTH {
            if position.can_play(col) {
                if position.is_winning_move(col) {
                    let score = (Position::WIDTH * Position::HEIGHT + 1 - position.nb_moves()) / 2;
                    scores[col as usize] = Some(score);
                } else {
                    let mut position_2: Position = position.clone();
                    position_2.play_col(col);
                    let score = -self.solve(&position_2, strong);
                    scores[col as usize] = Some(score)
                }
            }
        }
        scores
    }

    pub fn play(&mut self, position: &mut Position) -> Option<i32> {
        let mut max = i32::MIN;
        let mut cols = Vec::new();

        for (col, score) in self.analyze(position, true).iter().enumerate() {
            if let Some(score) = score {
                if score > &max {
                    max = *score;
                    cols = vec![col as i32];
                } else if *score == max {
                    cols.push(col as i32)
                }
            }
        }

        let col = cols.choose(&mut rand::thread_rng()); 

        match col {
            Some(col) => {
                position.play_col(*col);
                Some(*col)
            },
            None => None
        }
    }

    pub fn get_node_count(&self) -> u64 {
        return self.node_count;
    }

    pub fn reset(&mut self) {
        self.node_count = 0;
        self.table.reset()
    }

    pub fn load_book(&mut self, path: &str) {
        self.book.load(path).expect("Failed to load book");
    }
}

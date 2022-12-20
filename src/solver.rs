use crate::opening_book::OpeningBook;
use crate::position::Position; 
use crate::move_sorter::MoveSorter;
use crate::transposition_table::*;

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
            column_order[i as usize] = Position::WIDTH/2 - (i+1)*(i%2*2-1)/2
        }

        return Solver { node_count: 0, column_order, table: TranspositionTable::new(Solver::TABLE_SIZE), book: OpeningBook::new(Position::WIDTH, Position::HEIGHT) }
    }

    const TABLE_SIZE: u8 = 24;
    const INVALID_MOVE: i32 = -1000;

    fn negamax(&mut self, p: &Position, mut alpha: i32, mut beta: i32) -> i32 {
        assert!(alpha < beta);
        assert!(!p.can_win_next()); 

        self.node_count += 1;

        let possible: u64 = p.possible_non_losing_moves();
        if possible == 0 {
            return -(Position::WIDTH * Position::HEIGHT - p.nb_moves()) / 2;
        }

        if p.nb_moves() >= Position::WIDTH * Position::HEIGHT - 2 {
            return 0;
        }

        let mut min: i32 = -(Position::WIDTH * Position::HEIGHT - 2 - p.nb_moves())/2;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        let mut max: i32 = (Position::WIDTH * Position::HEIGHT - 1 - p.nb_moves())/2;
        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }

        if let Some(val) = self.book.get(&p) {
            if val == 0 {
                dbg!(val); 
            } else {
                return val as i32 + Position::MIN_SCORE - 1;
            }
        }
        
        let key: u64 = p.key();
        if let Some(val) = self.table.get(key) {
            if val == 0 { // !?!?!?!!?
                dbg!(val);
            } else {
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
        }

        let mut moves: MoveSorter = MoveSorter::new();
        for i in (0..Position::WIDTH).rev() {
            let m: u64 = possible & Position::column_mask(self.column_order[i as usize]);
            if m != 0 {
                moves.add(m, p.move_score(m));
            }
        }

        while let Some(next) = moves.get_next() {
            let mut p2: Position = p.clone();
            p2.play(next);
            let score: i32 = -self.negamax(&p2, -beta, -alpha);

            if score >= beta {
                self.table.put(key, (score + Position::MAX_SCORE - 2 * Position::MIN_SCORE + 2) as i8);
                return score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        self.table.put(key, (alpha - Position::MIN_SCORE + 1) as i8);
        return alpha;
    }

    pub fn solve(&mut self, p: &Position, strong: bool) -> i32 {
        if p.can_win_next() {
            return (Position::WIDTH*Position::HEIGHT + 1 - p.nb_moves()) / 2;
        }

        let mut min = -(Position::WIDTH*Position::HEIGHT - p.nb_moves()) / 2;
        let mut max = (Position::WIDTH*Position::HEIGHT + 1 - p.nb_moves()) / 2;

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

            let r = self.negamax(p, med, med + 1);
            if r <= med {
                max = r
            } else {
                min = r
            }
        }
        return min
    } 

    pub fn analyze(&mut self, p: &Position, strong: bool) -> Vec<i32> {
        let mut scores: Vec<i32> = vec![Solver::INVALID_MOVE; Position::WIDTH as usize]; 
        for col in 0..Position::WIDTH {
            if p.can_play(col) {
                if p.is_winning_move(col) {
                    scores[col as usize] = (Position::WIDTH * Position::HEIGHT + 1 - p.nb_moves()) / 2;
                } else {
                    let mut p2: Position = p.clone();
                    p2.play_col(col);
                    scores[col as usize] = -self.solve(&p2, strong); 
                }
            }
        }
        return scores; 
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
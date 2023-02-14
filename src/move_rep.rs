use std::fmt;

use crate::{Disk, Position, Solver, State};

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub turn: i32,
    pub disk: Disk,
    pub col: i32,
    pub row: i32,
    pub is_blunder: bool,
    pub is_severe_blunder: bool,
    pub is_only_not_blunder: bool,
    pub evaluation: i32,
}

impl Move {
    pub fn new(position: &mut Position, solver: &mut Solver, col: i32) -> Self {
        assert!(position.can_play(col));

        let old_evaluation = solver.evaluate(&position, false);

        let mut new_position = position.clone();
        new_position.play(col);

        let new_evaluation = -solver.evaluate(&new_position, false);

        let evaluation = new_evaluation - old_evaluation;

        let turn = position.nb_moves() + 1;
        let disk = position.disk_to_play();
        let row = position.playable_row_in_col(col).unwrap();

        let mut is_blunder = false;
        let mut is_severe_blunder = false;

        if !position.is_winning_move(col) {
            is_blunder = (old_evaluation > 0 && new_evaluation == 0)
                || (old_evaluation == 0 && new_evaluation < 0);
            is_severe_blunder = old_evaluation > 0 && new_evaluation < 0;
        }

        let mut is_only_not_blunder = true;

        if old_evaluation < 0 || is_blunder {
            is_only_not_blunder = false
        } else {
            for (i, eval) in solver.analyze(position, false).iter().enumerate() {
                if i == col as usize {
                    continue;
                }

                if let Some(e) = eval {
                    if (old_evaluation > 0 && *e > 0) || (old_evaluation == 0 && *e >= 0) {
                        is_only_not_blunder = false;
                    }
                }
            }
        }

        Self {
            turn,
            disk,
            col,
            row,
            is_blunder,
            is_severe_blunder,
            is_only_not_blunder,
            evaluation,
        }
    }

    fn get_col_char(&self) -> char {
        let c;
        match self.disk {
            Disk::X => {
                c = match self.col {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    3 => 'D',
                    4 => 'E',
                    5 => 'F',
                    6 => 'G',
                    _ => panic!(),
                };
            }
            Disk::O => {
                c = match self.col {
                    0 => 'a',
                    1 => 'b',
                    2 => 'c',
                    3 => 'd',
                    4 => 'e',
                    5 => 'f',
                    6 => 'g',
                    _ => panic!(),
                };
            }
        }

        c
    }

    fn get_row_char(&self) -> char {
        self.row.to_string().chars().next().unwrap()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push(self.get_col_char());
        s.push(self.get_row_char());
        if self.is_blunder {
            s.push('?')
        } else if self.is_severe_blunder {
            s.push_str("??");
        } else if self.is_only_not_blunder {
            s.push('!')
        }

        write!(f, "{}", s)
    }
}

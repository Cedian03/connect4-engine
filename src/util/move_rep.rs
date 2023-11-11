// use std::fmt;

// use crate::{Disk, Position, Solver};

// #[derive(Clone, Debug)]
// pub struct Move {
//     pub col: usize,
//     pub row: usize,
//     pub evaluation: Option<i32>,
//     pub flags: Flags,
// }

// #[derive(Clone, Debug)]
// pub struct Flags {
//     pub is_blunder: bool,
//     pub is_severe_blunder: bool,
//     // pub is_only_non_blunder: bool,
// }

// impl Move {
//     pub fn new(position: &mut Position, solver: &mut Solver, col: usize) -> Self {
//         assert!(position.can_play(col));

//         let old_evaluation = solver.evaluate(&position);

//         let mut new_position = position.clone();
//         new_position.play_col(col);

//         let new_evaluation = -solver.evaluate(&new_position);

//         let evaluation = Some(new_evaluation - old_evaluation);

//         let disk = position.disk_to_play();
//         let row = position.possible_row_in_col(col).unwrap();

//         let mut is_blunder = false;
//         let mut is_severe_blunder = false;

//         if !position.is_winning_move(col) {
//             is_blunder = (old_evaluation > 0 && new_evaluation == 0)
//                 || (old_evaluation == 0 && new_evaluation < 0);
//             is_severe_blunder = old_evaluation > 0 && new_evaluation < 0;
//         }

//         let mut is_only_non_blunder = true;

//         if old_evaluation < 0 || is_blunder {
//             is_only_non_blunder = false
//         } else {
//             for (i, eval) in solver.anazlyse(position).iter().enumerate() {
//                 if i == col as usize {
//                     continue;
//                 }

//                 if let Some(e) = eval {
//                     if (old_evaluation > 0 && *e > 0) || (old_evaluation == 0 && *e >= 0) {
//                         is_only_non_blunder = false;
//                     }
//                 }
//             }
//         }

//         Self {
//             col,
//             row,
//             evaluation,
//             flags: Flags {
//                 is_blunder,
//                 is_severe_blunder,
//             },
//         }
//     }
// }

// impl fmt::Display for Move {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", Move::col_to_char(self.col).unwrap())?;
//         write!(f, "{}", Move::row_to_char(self.row).unwrap())?;
//         write!(f, "{}", self.flags)
//     }
// }

// impl fmt::Display for Flags {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match (
//             self.is_blunder,
//             self.is_severe_blunder,
//             false // self.is_only_non_blunder,
//         ) {
//             (false, false, false) => Ok(()),
//             (true, false, false) => write!(f, "?"),
//             (true, true, false) => write!(f, "??"),
//             (false, false, true) => write!(f, "!"),
//             _ => unreachable!(),
//         }
//     }
// }

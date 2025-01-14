mod error;
mod magic;
mod bit_board;
mod solver;

pub use error::{Error, Result};
pub use bit_board::BitBoard;
pub use solver::{OpeningBook, Solver};

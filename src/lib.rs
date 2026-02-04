pub mod util;

mod board;
mod error;
mod solver;

pub use board::{Board, Disk};
pub use error::{Error, Result};
pub use solver::{OpeningBook, Solver};

pub type DefaultBoard = Board<7, 6>;

mod error;
mod magic;
mod position;
mod solver;

pub use error::{Error, Result};
pub use position::Position;
pub use solver::{OpeningBook, Solver};

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub use crate::position::Position;
pub use crate::solver::Solver;
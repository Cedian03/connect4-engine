mod board;
mod error;
mod magic;
mod solver;

pub use board::{BitBoard, Board, Cell, Col, Disk, Row};
pub use error::{Error, Result};
pub use magic::{AsBitBoard, BitMask, BitMaskOps};
pub use solver::{OpeningBook, Solver};

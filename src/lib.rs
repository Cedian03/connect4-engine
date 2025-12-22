mod board;
mod error;
mod magic;
mod solver;
pub mod util;

use std::{fmt, ops};

pub use board::Board;
pub use error::{Error, Result};
pub use magic::{AsBitBoard, BitMask, BitMaskOps};
pub use solver::{OpeningBook, Solver};

pub type Col = usize;
pub type Row = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disk {
    X,
    O,
}

impl ops::Not for Disk {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Disk::X => write!(f, "X"),
            Disk::O => write!(f, "O"),
        }
    }
}

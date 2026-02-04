use std::{fmt, ops};

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

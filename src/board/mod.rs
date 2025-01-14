mod bit_board;

pub use bit_board::BitBoard;

use std::{fmt, ops};

use crate::magic::*;

#[derive(Default)]
pub struct Board<const W: usize, const H: usize>(BitBoard<W, H>)
where
    BitBoard<W, H>: AsBitMask;

impl<const W: usize, const H: usize> Board<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    pub fn new() -> Self {
        Self(BitBoard::new())
    }

    pub fn play(&mut self, col: Col<W>) -> Result<Cell<W, H>, ()> {
        self.can_play_col(*col)
            .then(|| {
                let row = Row(self.possible_row_in_col(*col));
                self.play_col(*col);
                Cell(col, row)
            })
            .ok_or(())
    }

    pub fn play_sequence(&mut self, sequence: impl IntoIterator<Item = Col<W>>) -> Result<(), ()> {
        for col in sequence.into_iter() {
            self.play(col)?;
        }
        Ok(())
    }

    pub fn get(&self, col: Col<W>, row: Row<H>) -> Option<Disk> {
        let cell_mask = BitBoard::cell_mask(*col, *row);

        if (cell_mask & self.mask) != 0.into() {
            if (self.half_turn() % 2 == 0) ^ ((cell_mask & self.curr) == 0.into()) {
                Some(Disk::X)
            } else {
                Some(Disk::O)
            }
        } else {
            None
        }
    }

    pub fn to_play(&self) -> Disk {
        match self.half_turn() % 2 {
            0 => Disk::X,
            1 => Disk::O,
            _ => unreachable!(),
        }
    }
}

impl<const W: usize, const H: usize> std::ops::Deref for Board<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    type Target = BitBoard<W, H>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const W: usize, const H: usize> std::ops::DerefMut for Board<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const W: usize, const H: usize> fmt::Display for Board<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..H).rev() {
            write!(f, "{} ", Row::<H>(row))?;

            for col in 0..W {
                if col != 0 {
                    write!(f, " ")?
                }

                match self.get(Col(col), Row(row)) {
                    Some(disk) => write!(f, "{}", disk)?,
                    None => write!(f, ".")?,
                }
            }

            write!(f, "\n")?;
        }

        write!(f, " ")?;

        for col in 0..W {
            write!(f, " {}", Col::<W>(col))?
        }

        Ok(())
    }
}

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

pub struct Cell<const W: usize, const H: usize>(Col<W>, Row<H>);

impl<const W: usize, const H: usize> fmt::Display for Cell<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

/// **Note**: Zero indexed
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Col<const W: usize>(usize);

impl<const W: usize> Col<W> {
    fn as_char(self) -> char {
        (b'A' + self.0 as u8) as char
    }
}

impl<const W: usize> TryFrom<usize> for Col<W> {
    type Error = (); // TODO

    fn try_from(col: usize) -> std::result::Result<Self, Self::Error> {
        (col < W).then(|| Self(col)).ok_or(())
    }
}

impl<const W: usize> TryFrom<char> for Col<W> {
    type Error = (); // TODO

    fn try_from(col: char) -> std::result::Result<Self, Self::Error> {
        (col as usize)
            .checked_sub('A' as usize)
            .filter(|&col| col < W)
            .map(|col| Self(col))
            .ok_or(())
    }
}

impl<const W: usize> std::ops::Deref for Col<W> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const W: usize> fmt::Display for Col<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.alternate() {
            false => write!(f, "{}", self.as_char()),
            true => write!(f, "{}", self.as_char().to_ascii_uppercase()),
        }
    }
}

/// **Note**: Zero indexed
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row<const H: usize>(usize);

impl<const H: usize> Row<H> {
    fn as_char(self) -> char {
        (b'1' + self.0 as u8) as char
    }
}

impl<const H: usize> TryFrom<usize> for Row<H> {
    type Error = (); // TODO

    fn try_from(row: usize) -> std::result::Result<Self, Self::Error> {
        (row < H).then(|| Self(row)).ok_or(())
    }
}

impl<const H: usize> TryFrom<char> for Row<H> {
    type Error = (); // TODO

    fn try_from(row: char) -> std::result::Result<Self, Self::Error> {
        (row as usize)
            .checked_sub('1' as usize)
            .filter(|&row| row < H)
            .map(|row| Self(row))
            .ok_or(())
    }
}

impl<const H: usize> std::ops::Deref for Row<H> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const H: usize> fmt::Display for Row<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

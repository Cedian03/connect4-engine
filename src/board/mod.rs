mod bit_board;

use std::fmt;

use crate::{AsBitBoard, Col, Disk, Row};

pub use bit_board::BitBoard;

#[derive(Clone, Copy, Default)]
pub struct Board<const W: usize = 7, const H: usize = 6>(BitBoard<W, H>)
where
    Self: AsBitBoard;

impl<const W: usize, const H: usize> Board<W, H>
where
    Self: AsBitBoard,
{
    pub fn new() -> Self {
        Self(BitBoard::new())
    }

    pub fn play(&mut self, col: Col) -> Result<Row, ()> {
        if !self.can_play(col) {
            return Err(());
        }

        let row = self.possible_row_in_col(col);
        self.play_col(col);
        return Ok(row);
    }

    pub fn play_sequence(&mut self, sequence: impl IntoIterator<Item = Col>) -> Result<(), ()> {
        for col in sequence.into_iter() {
            self.play(col)?;
        }
        Ok(())
    }

    pub fn can_play(&self, col: usize) -> bool {
        self.can_play_col(col)
    }

    pub fn get(&self, col: usize, row: usize) -> Option<Disk> {
        let cell_mask = BitBoard::cell_mask(col, row);

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
    Self: AsBitBoard,
{
    type Target = BitBoard<W, H>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const W: usize, const H: usize> std::ops::DerefMut for Board<W, H>
where
    Self: AsBitBoard,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const W: usize, const H: usize> fmt::Display for Board<W, H>
where
    Self: AsBitBoard,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..H).rev() {
            if row != 0 {
                write!(f, "\n")?;
            }

            for col in 0..W {
                if col != 0 {
                    write!(f, " ")?
                }

                match self.get(col, row) {
                    Some(disk) => write!(f, "{}", disk)?,
                    None => write!(f, ".")?,
                }
            }
        }

        Ok(())
    }
}

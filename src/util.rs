use crate::{Error, Position, Result};

pub fn col_to_char(col: usize) -> Result<char> {
    (col < Position::WIDTH)
        .then(|| char::from(col as u8 + 'A' as u8))
        .ok_or_else(|| Error::Convert(format!("Invalid column index: {col}")))
}

pub fn row_to_char(row: usize) -> Result<char> {
    (row < Position::HEIGHT)
        .then(|| char::from(row as u8 + '1' as u8))
        .ok_or_else(|| Error::Convert(format!("Invalid row index: {row}")))
}

pub fn char_to_col(ch: char) -> Result<usize> {
    (ch as usize)
        .checked_sub('A' as usize)
        .filter(|&col| (col < Position::WIDTH))
        .ok_or_else(|| Error::Convert(format!("Invalid column char: '{ch}'")))
}

pub fn char_to_row(ch: char) -> Result<usize> {
    (ch as usize)
        .checked_sub('1' as usize)
        .filter(|&row| (row < Position::HEIGHT))
        .ok_or_else(|| Error::Convert(format!("Invalid row char: '{ch}'")))
}

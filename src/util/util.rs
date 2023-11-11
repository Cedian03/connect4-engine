use crate::position::Position;

pub fn col_to_char(col: usize) -> Option<char> {
    (col < Position::WIDTH).then(|| char::from(col as u8 + 'A' as u8))
}

pub fn row_to_char(row: usize) -> Option<char> {
    (row < Position::HEIGHT).then(|| char::from(row as u8 + '1' as u8))
}

pub fn char_to_col(ch: char) -> Option<usize> {
    let col = (ch as usize).checked_sub('A' as usize)?; 
    (col < Position::WIDTH).then_some(col)
}

pub fn char_to_row(ch: char) -> Option<usize> {
    let row = (ch as usize).checked_sub('1' as usize)?; 
    (row < Position::WIDTH).then_some(row)
}
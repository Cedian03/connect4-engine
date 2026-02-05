use std::fmt;

use num_traits::{PrimInt, Zero};

use crate::util::char_to_col;

use super::{magic::*, Disk};

#[derive(Debug, Copy, Clone, Default)]
pub struct Board<const W: usize = 7, const H: usize = 6>
where
    Board<W, H>: AsBitBoard,
{
    pub(crate) curr: BitMask<W, H>,
    pub(crate) mask: BitMask<W, H>,
}

impl<const W: usize, const H: usize> Board<W, H>
where
    Board<W, H>: AsBitBoard,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, col: usize) -> Result<(usize, bool), String> {
        assert!(
            col < W,
            "column out of bounds: column is {} but width is {}",
            col,
            W
        );

        if !self.can_play_column(col) {
            return Err("column is full".to_owned());
        }

        let row = self.possible_row_in_column(col);
        let did_win = self.is_winning_column(col);

        self.play_column(col);

        Ok((row, did_win))
    }

    pub fn try_play_sequence<I: IntoIterator<Item = usize>>(
        &mut self,
        sequence: I,
    ) -> Result<(), String> {
        sequence
            .into_iter()
            .try_for_each(|col| self.play(col).map(|_| ()))
    }

    pub fn try_play_string(&mut self, string: &str) -> Result<(), String> {
        let sequence = string
            .chars()
            .map(|ch| char_to_col(ch).ok_or(format!("unable to convert `{ch}` to a column index")))
            .collect::<Result<Vec<_>, _>>()?;

        self.try_play_sequence(sequence)
    }

    pub fn disk_to_play(&self) -> Disk {
        match self.half_turn() % 2 {
            0 => Disk::X,
            1 => Disk::O,
            _ => unreachable!(),
        }
    }

    pub fn get_cell(&self, col: usize, row: usize) -> Option<Disk> {
        assert!(
            col < W,
            "column out of bounds: column is {} but width is {}",
            col,
            W
        );

        assert!(
            row < H,
            "row out of bounds: row is {} but height is {}",
            row,
            H
        );

        let cell_mask = Self::cell_mask(col, row);

        if (cell_mask & self.mask).is_zero() {
            return None;
        }

        let disk_to_play = self.disk_to_play();
        if (cell_mask & self.curr).is_zero() {
            Some(!disk_to_play)
        } else {
            Some(disk_to_play)
        }
    }

    pub fn half_turn(&self) -> i32 {
        self.mask.count_ones() as i32
    }

    pub fn turn(&self) -> i32 {
        self.half_turn() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.mask.is_zero()
    }

    pub const fn width(&self) -> usize {
        W
    }

    pub const fn height(&self) -> usize {
        H
    }

    /// Will corrupt the board state if `col` is not less than `W` or `col` is not possible.
    pub(crate) fn play_column(&mut self, col: usize) {
        Self::debug_assert_column_bound(col);
        self.play_mask(self.possible_mask_column(col))
    }

    /// TODO: precondition comment
    pub(crate) fn play_mask(&mut self, mask: BitMask<W, H>) {
        self.curr ^= self.mask;
        self.mask |= mask;
    }

    pub(crate) fn possible_non_losing_moves(&self) -> BitMask<W, H> {
        let mut possible_mask = self.possible_mask();
        let opponent_win = self.opponent_winning_cells();
        let forced_moves = possible_mask & opponent_win;

        if forced_moves.is_not_zero() {
            if forced_moves.count_ones() > 1 {
                return 0.into();
            } else {
                possible_mask = forced_moves;
            }
        }

        possible_mask & !(opponent_win >> 1)
    }

    /// Will return nonsense if `col` is not less than `W`.
    pub fn can_play_column(&self, col: usize) -> bool {
        self.possible_mask_column(col).is_not_zero()
    }

    /// Will return nonsense if `col` is not less than `W`.
    fn possible_row_in_column(&self, col: usize) -> usize {
        Self::debug_assert_column_bound(col);
        (self.possible_mask_column(col).trailing_zeros() % (H as u32 + 1)) as usize
    }

    /// Will return nonsense if `col` is not less than `W`.
    fn possible_mask_column(&self, col: usize) -> BitMask<W, H> {
        Self::debug_assert_column_bound(col);
        self.possible_mask() & Board::column_mask(col)
    }

    fn possible_mask(&self) -> BitMask<W, H> {
        (self.mask + Self::bottom_mask()) & Self::board_mask()
    }

    pub fn can_win_next(&self) -> bool {
        (self.winning_cells() & self.possible_mask()).is_not_zero()
    }

    /// Will return nonsense if `col` is not less than `W`.
    fn is_winning_column(&self, col: usize) -> bool {
        Self::debug_assert_column_bound(col);
        (self.winning_cells() & self.possible_mask() & Board::column_mask(col)).is_not_zero()
    }

    fn winning_cells(&self) -> BitMask<W, H> {
        Board::compute_winning_cells(self.curr, self.mask)
    }

    fn opponent_winning_cells(&self) -> BitMask<W, H> {
        Board::compute_winning_cells(self.curr ^ self.mask, self.mask)
    }

    /// Will return nonsense if `board` and `mask` does not form a valid position.
    pub(crate) fn compute_winning_cells(
        board: BitMask<W, H>,
        mask: BitMask<W, H>,
    ) -> BitMask<W, H> {
        // Vertical
        let mut r = (board << 1) & (board << 2) & (board << 3);

        // Horizontal
        let mut b = (board << (H + 1)) & (board << 2 * (H + 1));
        r |= b & (board << 3 * (H + 1));
        r |= b & (board >> (H + 1));
        b = (board >> (H + 1)) & (board >> 2 * (H + 1));
        r |= b & (board << (H + 1));
        r |= b & (board >> 3 * (H + 1));

        // Diagonal 1
        b = (board << H) & (board << 2 * H);
        r |= b & (board << 3 * H);
        r |= b & (board >> H);
        b = (board >> H) & (board >> 2 * H);
        r |= b & (board << H);
        r |= b & (board >> 3 * H);

        // Diagonal 2
        b = (board << (H + 2)) & (board << 2 * (H + 2));
        r |= b & (board << 3 * (H + 2));
        r |= b & (board >> (H + 2));
        b = (board >> (H + 2)) & (board >> 2 * (H + 2));
        r |= b & (board << (H + 2));
        r |= b & (board >> 3 * (H + 2));

        r & (Self::board_mask() ^ mask)
    }

    pub(crate) fn key(&self) -> BitMask<W, H> {
        self.curr + self.mask + Self::board_mask()
    }

    pub(crate) fn key_3(&self) -> BitMask<W, H> {
        let mut key_forward = <BitMask<W, H>>::from(0);
        for col in 0..W {
            self.partial_key_3(&mut key_forward, col);
        }

        let mut key_reverse = <BitMask<W, H>>::from(0);
        for col in (0..W).rev() {
            self.partial_key_3(&mut key_reverse, col);
        }

        key_forward.min(key_reverse) / <BitMask<W, H>>::from(3)
    }

    fn partial_key_3(&self, key: &mut BitMask<W, H>, col: usize) {
        let mut board = Self::column_mask(col) & Self::bottom_mask();

            while (board & self.mask).is_not_zero() {
                *key *= <BitMask<W, H>>::from(3);

                if (board & self.curr).is_not_zero() {
                    *key += <BitMask<W, H>>::from(1);
                } else {
                    *key += <BitMask<W, H>>::from(2);
                }

                board <<= 1;
            }

            *key *= <BitMask<W, H>>::from(3);
    }

    /// Will return nonsense if `col` is not less than `W` or `row` is not less than `H`.
    fn cell_mask(col: usize, row: usize) -> BitMask<W, H> {
        Self::debug_assert_column_bound(col);
        Self::debug_assert_row_bound(row);
        <BitMask<W, H>>::from(1) << (col * (H + 1) + row)
    }

    /// Will return nonsense if `col` is not less than `W`.
    pub(crate) fn column_mask(col: usize) -> BitMask<W, H> {
        Self::debug_assert_column_bound(col);
        ((<BitMask<W, H>>::from(1) << H) - <BitMask<W, H>>::from(1)) << col * (H + 1)
    }

    fn bottom_mask() -> BitMask<W, H> {
        let mut mask = <BitMask<W, H>>::from(0);

        for _ in 0..W {
            mask <<= H + 1;
            mask |= <BitMask<W, H>>::from(1);
        }

        mask
    }

    fn board_mask() -> BitMask<W, H> {
        Self::bottom_mask() * ((<BitMask<W, H>>::from(1) << H) - <BitMask<W, H>>::from(1))
    }

    #[track_caller]
    fn debug_assert_column_bound(col: usize) {
        debug_assert!(
            col < W,
            "column out of bounds: column is {} but width is {}",
            col,
            W
        );
    }

    #[track_caller]
    fn debug_assert_row_bound(row: usize) {
        debug_assert!(
            row < H,
            "row out of bounds: row is {} but height is {}",
            row,
            H
        );
    }
}

impl fmt::Display for Board<7, 6> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..self.height()).rev() {
            if row != self.height() - 1 {
                write!(f, "\n")?
            }

            for col in 0..self.width() {
                if col != 0 {
                    write!(f, " ")?
                }

                match self.get_cell(col, row) {
                    Some(disk) => write!(f, "{}", disk)?,
                    None => write!(f, ".")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of_standard_board_is_16_bytes() {
        assert_eq!(size_of::<Board<7, 6>>(), 16);
    }

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::<7, 6>::new();
        assert!(board.is_empty());

        for row in 0..board.height() {
            for col in 0..board.width() {
                assert!(board.get_cell(col, row).is_none())
            }
        }
    }

    #[test]
    fn test_() {
        let mut board = Board::<7, 6>::new();
        let _ = board.play(3).unwrap();
        let _ = board.play(3).unwrap();
        let _ = board.play(3).unwrap();

        assert_eq!(board.get_cell(3, 0), Some(Disk::X));
        assert_eq!(board.get_cell(3, 1), Some(Disk::O));
        assert_eq!(board.get_cell(3, 2), Some(Disk::X));
    }

    #[test]
    #[should_panic(expected = "column out of bounds")]
    #[cfg(debug_assertions)]
    fn test_get_disk_column_out_of_bounds() {
        let board = Board::<7, 6>::new();
        let _ = board.get_cell(7, 0);
    }

    #[test]
    #[should_panic(expected = "row out of bounds")]
    #[cfg(debug_assertions)]
    fn test_get_disk_row_out_of_bounds() {
        let board = Board::<7, 6>::new();
        let _ = board.get_cell(0, 6);
    }

    #[test]
    fn test_column_top_mask() {
        assert_eq!(
            Board::<7, 6>::column_top_mask(0),
            0b0000000_0000000_0000000_0000000_0000000_0000000_0100000
        );
        assert_eq!(
            Board::<7, 6>::column_top_mask(6),
            0b0100000_0000000_0000000_0000000_0000000_0000000_0000000
        );
    }

    #[test]
    fn test_cell_mask() {
        assert_eq!(
            Board::<7, 6>::cell_mask(0, 0),
            0b0000000_0000000_0000000_0000000_0000000_0000000_0000001
        );
        assert_eq!(
            Board::<7, 6>::cell_mask(6, 5),
            0b0100000_0000000_0000000_0000000_0000000_0000000_0000000
        );
    }

    #[test]
    fn test_column_mask() {
        assert_eq!(
            Board::<7, 6>::column_mask(0),
            0b0000000_0000000_0000000_0000000_0000000_0000000_0111111
        );
        assert_eq!(
            Board::<7, 6>::column_mask(6),
            0b0111111_0000000_0000000_0000000_0000000_0000000_0000000
        );
    }

    #[test]
    fn test_bottom_mask() {
        assert_eq!(
            Board::<7, 6>::bottom_mask(),
            0b0000001_0000001_0000001_0000001_0000001_0000001_0000001
        );
    }

    #[test]
    fn test_board_mask() {
        assert_eq!(
            Board::<7, 6>::board_mask(),
            0b0111111_0111111_0111111_0111111_0111111_0111111_0111111
        );
    }
}

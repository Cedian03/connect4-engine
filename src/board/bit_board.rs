use num_traits::{PrimInt, Zero};

use crate::{board::Board, magic::*};

#[derive(Copy, Clone)]
pub struct BitBoard<const W: usize = 7, const H: usize = 6>
where
    Board<W, H>: AsBitBoard,
{
    pub(crate) curr: BitMask<W, H>,
    pub(crate) mask: BitMask<W, H>,
}

impl BitBoard {
    pub fn standard() -> Self {
        Self::default()
    }
}

impl<const W: usize, const H: usize> BitBoard<W, H>
where
    Board<W, H>: AsBitBoard,
{
    // pub const BOTTOM_MASK: BitMask<W, H> = Self::bottom_mask(); // TODO
    // pub const BOARD_MASK: BitMask<W, H> = Self::board_mask(); // TODO

    pub fn new() -> Self {
        Self::default()
    }

    pub const fn width(&self) -> usize {
        W
    }

    pub const fn height(&self) -> usize {
        H
    }

    pub const fn curr(&self) -> BitMask<W, H> {
        self.curr
    }

    pub const fn mask(&self) -> BitMask<W, H> {
        self.mask
    }

    pub fn play_col(&mut self, col: usize) {
        self.play_mask(self.possible_mask_col(col))
    }

    pub fn play_mask(&mut self, mask: BitMask<W, H>) {
        self.curr ^= self.mask;
        self.mask |= mask;
    }

    pub fn turn(&self) -> i32 {
        self.half_turn() / 2
    }

    pub fn half_turn(&self) -> i32 {
        self.mask.count_ones() as i32
    }

    pub fn possible_non_losing_moves(&self) -> BitMask<W, H> {
        let mut possible_mask = self.possible_mask();
        let opponent_win = self.opponent_winning_cells();
        let forced_moves = possible_mask & opponent_win;

        if !(forced_moves.is_zero()) {
            if forced_moves.count_ones() > 1 {
                return 0.into();
            } else {
                possible_mask = forced_moves;
            }
        }

        possible_mask & !(opponent_win >> 1)
    }

    pub fn can_play_col(&self, col: usize) -> bool {
        (self.mask & BitBoard::col_top_mask(col)).is_zero()
    }

    pub fn possible_row_in_col(&self, col: usize) -> usize {
        (self.possible_mask_col(col).trailing_zeros() % (H as u32 + 1)) as usize
    }

    pub fn possible_mask_col(&self, col: usize) -> BitMask<W, H> {
        self.possible_mask() & BitBoard::col_mask(col)
    }

    pub fn possible_mask(&self) -> BitMask<W, H> {
        (self.mask + Self::bottom_mask()) & Self::board_mask()
    }

    pub fn can_win_next(&self) -> bool {
        !((self.winning_cells() & self.possible_mask()).is_zero())
    }

    pub fn is_winning_col(&self, col: usize) -> bool {
        !((self.winning_cells() & self.possible_mask() & BitBoard::col_mask(col)).is_zero())
    }

    pub fn winning_cells(&self) -> BitMask<W, H> {
        BitBoard::compute_winning_cells(self.curr, self.mask)
    }

    pub fn opponent_winning_cells(&self) -> BitMask<W, H> {
        BitBoard::compute_winning_cells(self.curr ^ self.mask, self.mask)
    }

    pub fn compute_winning_cells(board: BitMask<W, H>, mask: BitMask<W, H>) -> BitMask<W, H> {
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

    pub fn key(&self) -> BitMask<W, H> {
        self.curr + self.mask + Self::board_mask()
    }

    pub fn key_3(&self) -> BitMask<W, H> {
        let mut key_forward = 0.into();
        for col in 0..W {
            self.partial_key_3(&mut key_forward, col);
        }

        let mut key_reverse = 0.into();
        for col in (0..W).rev() {
            self.partial_key_3(&mut key_reverse, col);
        }

        if key_forward < key_reverse {
            key_forward / 3.into()
        } else {
            key_reverse / 3.into()
        }
    }

    fn partial_key_3(&self, key: &mut BitMask<W, H>, col: usize) {
        let mut board = Self::col_top_mask(col);

        while (board & self.mask) != 0.into() {
            *key *= 3.into();

            if (board & self.curr) != 0.into() {
                *key += 1.into();
            } else {
                *key += 2.into();
            }

            board <<= 1;
        }

        *key *= 3.into();
    }

    pub fn col_top_mask(col: usize) -> BitMask<W, H> {
        <BitMask<W, H>>::from(1) << (col * (H + 1) + (H - 1))
    }

    pub fn cell_mask(col: usize, row: usize) -> BitMask<W, H> {
        <BitMask<W, H>>::from(1) << (col * (H + 1) + row)
    }

    pub fn col_bot_mask(col: usize) -> BitMask<W, H> {
        <BitMask<W, H>>::from(1) << col * (H + 1)
    }

    pub fn col_mask(col: usize) -> BitMask<W, H> {
        ((<BitMask<W, H>>::from(1) << H) - 1.into()) << col * (H + 1)
    }

    // TODO: constify
    fn bottom_mask() -> BitMask<W, H> {
        let mut mask = 0.into();

        let mut i = 0;
        while i < W {
            mask <<= H + 1;
            mask |= 1.into();

            i += 1;
        }

        mask
    }

    // TODO: constify
    fn board_mask() -> BitMask<W, H> {
        Self::bottom_mask() * ((<BitMask<W, H>>::from(1) << H) - 1.into())
    }
}

impl<const W: usize, const H: usize> Default for BitBoard<W, H>
where
    Board<W, H>: AsBitBoard,
{
    fn default() -> Self {
        Self {
            curr: 0.into(),
            mask: 0.into(),
        }
    }
}

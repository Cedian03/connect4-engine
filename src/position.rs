use crate::magic;

pub type BitMask =
    <magic::MagicStruct<{ Position::bits_required() }> as magic::MagicTrait>::MagicType;

#[derive(Clone, Debug)]
pub struct Position {
    position: BitMask,
    mask: BitMask,
    half_turn: i32,
}

impl Position {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub const BOTTOM_MASK: BitMask = Self::bottom_mask();
    pub const BOARD_MASK: BitMask = Self::board_mask();

    pub const fn new() -> Self {
        Self {
            position: 0,
            mask: 0,
            half_turn: 0,
        }
    }

    pub const fn position(&self) -> BitMask {
        self.position
    }

    pub const fn mask(&self) -> BitMask {
        self.mask
    }

    pub fn play_col(&mut self, col: usize) {
        self.play_mask(self.possible_mask_col(col))
    }

    pub fn play_mask(&mut self, mask: BitMask) {
        self.position ^= self.mask;
        self.mask |= mask;
        self.half_turn += 1;
    }

    pub fn turn(&self) -> i32 {
        self.half_turn / 2
    }

    pub fn half_turn(&self) -> i32 {
        self.half_turn
    }

    pub const fn possible_non_losing_moves(&self) -> BitMask {
        let mut possible_mask = self.possible();
        let opponent_win = self.opponent_winning_positions();
        let forced_moves = possible_mask & opponent_win;

        if forced_moves != 0 {
            if forced_moves.count_ones() > 1 {
                return 0;
            } else {
                possible_mask = forced_moves;
            }
        }

        possible_mask & !(opponent_win >> 1)
    }

    pub const fn can_play_col(&self, col: usize) -> bool {
        (self.mask & Position::col_top_mask(col)) == 0
    }

    pub const fn possible_mask_col(&self, col: usize) -> BitMask {
        self.possible() & Position::col_mask(col)
    }

    pub const fn possible(&self) -> BitMask {
        (self.mask + Position::BOTTOM_MASK) & Position::BOARD_MASK
    }

    pub const fn can_win_next(&self) -> bool {
        self.winning_positions() & self.possible() != 0
    }

    pub const fn is_winning_col(&self, col: usize) -> bool {
        self.winning_positions() & self.possible() & Position::col_mask(col) != 0
    }

    pub const fn winning_positions(&self) -> BitMask {
        Position::compute_winning_positions(self.position, self.mask)
    }

    pub const fn opponent_winning_positions(&self) -> BitMask {
        Position::compute_winning_positions(self.position ^ self.mask, self.mask)
    }

    pub const fn compute_winning_positions(position: BitMask, mask: BitMask) -> BitMask {
        // Vertical
        let mut r = (position << 1) & (position << 2) & (position << 3);

        // Horizontal
        let mut p = (position << (Position::HEIGHT + 1)) & (position << 2 * (Position::HEIGHT + 1));
        r |= p & (position << 3 * (Position::HEIGHT + 1));
        r |= p & (position >> (Position::HEIGHT + 1));
        p = (position >> (Position::HEIGHT + 1)) & (position >> 2 * (Position::HEIGHT + 1));
        r |= p & (position << (Position::HEIGHT + 1));
        r |= p & (position >> 3 * (Position::HEIGHT + 1));

        // Diagonal 1
        p = (position << Position::HEIGHT) & (position << 2 * Position::HEIGHT);
        r |= p & (position << 3 * Position::HEIGHT);
        r |= p & (position >> Position::HEIGHT);
        p = (position >> Position::HEIGHT) & (position >> 2 * Position::HEIGHT);
        r |= p & (position << Position::HEIGHT);
        r |= p & (position >> 3 * Position::HEIGHT);

        // Diagonal 2
        p = (position << (Position::HEIGHT + 2)) & (position << 2 * (Position::HEIGHT + 2));
        r |= p & (position << 3 * (Position::HEIGHT + 2));
        r |= p & (position >> (Position::HEIGHT + 2));
        p = (position >> (Position::HEIGHT + 2)) & (position >> 2 * (Position::HEIGHT + 2));
        r |= p & (position << (Position::HEIGHT + 2));
        r |= p & (position >> 3 * (Position::HEIGHT + 2));

        r & (Position::BOARD_MASK ^ mask)
    }

    pub const fn key(&self) -> BitMask {
        self.position + self.mask + Position::BOTTOM_MASK
    }

    pub fn key_3(&self) -> BitMask {
        let mut key_forward = 0;
        for col in 0..Position::WIDTH {
            self.partial_key_3(&mut key_forward, col);
        }

        let mut key_reverse = 0;
        for col in (0..Position::WIDTH).rev() {
            self.partial_key_3(&mut key_reverse, col);
        }

        if key_forward < key_reverse {
            key_forward / 3
        } else {
            key_reverse / 3
        }
    }

    fn partial_key_3(&self, key: &mut BitMask, col: usize) {
        let mut pos = 1 << (col * (Position::HEIGHT + 1));

        while pos & self.mask != 0 {
            *key *= 3;
            if pos & self.position != 0 {
                *key += 1
            } else {
                *key += 2
            }
            pos <<= 1;
        }
        *key *= 3;
    }

    pub const fn col_top_mask(col: usize) -> BitMask {
        1 << ((Position::HEIGHT - 1) + col * (Position::HEIGHT + 1))
    }

    pub const fn col_bot_mask(col: usize) -> BitMask {
        1 << col * (Position::HEIGHT + 1)
    }

    pub const fn col_mask(col: usize) -> BitMask {
        ((1 << Position::HEIGHT) - 1) << col * (Position::HEIGHT + 1)
    }

    const fn bottom_mask() -> BitMask {
        let mut mask = 0;

        let mut i = 0;
        while i < Position::WIDTH {
            mask <<= Self::HEIGHT + 1;
            mask |= 1;

            i += 1;
        }

        mask
    }

    const fn board_mask() -> BitMask {
        Self::bottom_mask() * ((1 << Position::HEIGHT) - 1)
    }

    pub const fn bits_required() -> usize {
        Self::WIDTH * (Self::HEIGHT + 1)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

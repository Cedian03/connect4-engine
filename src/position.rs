use std::default;
use std::fmt;
use std::ops;

use crate::prelude::*;
use crate::util::char_to_col;

const fn bottom_mask(w: usize, h: usize) -> u64 {
    if w == 1 {
        return 1;
    }

    (1 << (h + 1) * (w - 1)) | bottom_mask(w - 1, h)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Disk {
    X,
    O,
}

impl ops::Not for Disk {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Disk::X => Disk::O,
            Disk::O => Disk::X,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    position: u64,
    mask: u64,
    half_turn: i32,
}

impl Position {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;

    pub(crate) const AREA: i32 = (Position::WIDTH * Position::HEIGHT) as i32;

    pub(crate) const MIN_SCORE: i32 = -Position::AREA / 2 + 3;
    pub(crate) const MAX_SCORE: i32 = (Position::AREA + 1) / 2 - 3;

    const BOTTOM_MASK: u64 = bottom_mask(Position::WIDTH, Position::HEIGHT);
    const BOARD_MASK: u64 = Position::BOTTOM_MASK * ((1 << Position::HEIGHT) - 1);

    pub fn new() -> Self {
        Self::default()
    }

    pub fn play_seq<S: AsRef<str>>(&mut self, seq: S) -> Result<()> {
        for ch in seq.as_ref().chars() {
            self.play_col(char_to_col(ch)?)
        }
        Ok(())
    }

    pub fn play_col(&mut self, col: usize) {
        assert!(self.can_play(col));
        self.play_mask(self.possible_mask_col(col))
    }

    pub(crate) fn play_mask(&mut self, mask: u64) {
        self.position ^= self.mask;
        self.mask |= mask;
        self.half_turn += 1;
    }

    pub fn can_win_next(&self) -> bool {
        self.winning_positions() & self.possible() != 0
    }

    pub fn turn(&self) -> i32 {
        self.half_turn / 2
    }

    pub fn half_turn(&self) -> i32 {
        self.half_turn
    }

    pub(crate) fn key(&self) -> u64 {
        self.position + self.mask + Position::BOTTOM_MASK
    }

    pub(crate) fn key_3(&self) -> u64 {
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

    pub(crate) fn possible_non_losing_moves(&self) -> u64 {
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

    pub(crate) fn move_score(&self, mask: u64) -> u32 {
        Position::compute_winning_positions(self.position | mask, self.mask).count_ones()
    }

    pub fn can_play(&self, col: usize) -> bool {
        (self.mask & Position::top_mask_col(col)) == 0
    }

    pub fn is_winning_move(&self, col: usize) -> bool {
        self.winning_positions() & self.possible() & Position::column_mask(col) != 0
    }

    fn partial_key_3(&self, key: &mut u64, col: usize) {
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

    fn winning_positions(&self) -> u64 {
        Position::compute_winning_positions(self.position, self.mask)
    }

    fn opponent_winning_positions(&self) -> u64 {
        Position::compute_winning_positions(self.position ^ self.mask, self.mask)
    }

    fn possible(&self) -> u64 {
        (self.mask + Position::BOTTOM_MASK) & Position::BOARD_MASK
    }

    pub fn compute_winning_positions(position: u64, mask: u64) -> u64 {
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

    fn top_mask_col(col: usize) -> u64 {
        1 << ((Position::HEIGHT - 1) + col * (Position::HEIGHT + 1))
    }

    fn bottom_mask_col(col: usize) -> u64 {
        1 << col * (Position::HEIGHT + 1)
    }

    pub(crate) fn column_mask(col: usize) -> u64 {
        ((1 << Position::HEIGHT) - 1) << col * (Position::HEIGHT + 1)
    }

    pub fn disk_to_play(&self) -> Disk {
        match self.half_turn() % 2 {
            0 => Disk::X,
            1 => Disk::O,
            _ => unreachable!(),
        }
    }

    fn possible_mask_col(&self, col: usize) -> u64 {
        (self.mask + Position::bottom_mask_col(col)) & Position::column_mask(col)
    }

    pub fn possible_row_in_col(&self, col: usize) -> Option<usize> {
        let zeros = self.possible_mask_col(col).trailing_zeros();
        (zeros != 64).then(|| zeros as usize % (Position::HEIGHT + 1))
         
    }

    pub fn get(&self, col: usize, row: usize) -> Option<Disk> {
        assert!(col < Position::WIDTH);
        assert!(row < Position::HEIGHT);

        let mask = 1 << (col * (Position::HEIGHT + 1) + row);

        (self.mask & mask != 0).then(|| 
            match self.position & mask == 0 {
                true => !self.disk_to_play(),
                false => self.disk_to_play(),
        })
    }
}

impl default::Default for Position {
    fn default() -> Self {
        Self {
            position: 0,
            mask: 0,
            half_turn: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..Position::HEIGHT).rev() {
            write!(f, "{}", row + 1)?;
            for col in 0..Position::WIDTH {
                let ch = match self.get(col, row) {
                    Some(Disk::X) => 'X',
                    Some(Disk::O) => 'O',
                    None => '.',
                };
                write!(f, " {}", ch)?;
            }
            write!(f, "\n")?;
        }

        write!(f, " ")?;
        for i in 0..Position::WIDTH as u8 {
            let i = i + 'A' as u8;
            let c = char::from(i);
            write!(f, " {}", c)?;
        }

        Ok(())
    }
}


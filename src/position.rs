use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Disk {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    InProgress,
    Won(Disk),
    Draw,
}

#[derive(Clone, Debug)]
pub struct Position {
    current_position: u64,
    mask: u64,
    moves: i32,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut position = Position::new(); 

        for (i, c) in value.chars().enumerate() {
            let col = match c {
                'A'..='Z' => (c as i32) - ('A' as i32) + 1,
                'a'..='z' => (c as i32) - ('a' as i32) + 1,
                _ => panic!("Invalid sequence at position: {} ({})", i + 1, c),
            };
        
            if col >= 1 && col <= Position::WIDTH {
                position.play(col - 1);
            } else {
                panic!("Invalid column number: {}", col);
            }
        }

        position
    }
}

impl Position {
    pub fn new() -> Self {
        return Position {
            current_position: 0,
            mask: 0,
            moves: 0,
        };
    }

    pub const WIDTH: i32 = 7;
    pub const HEIGHT: i32 = 6;

    pub const MIN_SCORE: i32 = -(Position::WIDTH * Position::HEIGHT) / 2 + 3;
    pub const MAX_SCORE: i32 = ((Position::WIDTH * Position::HEIGHT) + 1) / 2 - 3;

    const BOTTOM_MASK: u64 = 0b0000001000000100000010000001000000100000010000001;
    const BOARD_MASK: u64 = 0b0111111011111101111110111111011111101111110111111;

    pub fn play(&mut self, col: i32) {
        self.play_mask((self.mask + Position::bottom_mask_col(col)) & Position::column_mask(col));
    }

    pub fn play_mask(&mut self, m: u64) {
        self.current_position ^= self.mask;
        self.mask |= m;
        self.moves += 1;
    }

    pub fn play_sequence(&mut self, sequence: &str) -> i32 {
        for (i, c) in sequence.chars().enumerate() {
            let col = c as i32 - '1' as i32;

            if col >= Position::WIDTH || !self.can_play(col) || self.is_winning_move(col) {
                return i as i32;
            }
            self.play(col);
        }
        return 0;
    }

    pub fn can_win_next(&self) -> bool {
        return self.winning_positions() & self.possible() != 0;
    }

    pub fn nb_moves(&self) -> i32 {
        return self.moves;
    }

    pub fn key(&self) -> u64 {
        return self.current_position + self.mask;
    }

    pub fn key_3(&self) -> u64 {
        let mut key_forward: u64 = 0;
        for col in 0..Position::WIDTH {
            self.partial_key_3(&mut key_forward, col)
        }

        let mut key_reverse: u64 = 0;
        for col in (0..Position::WIDTH).rev() {
            self.partial_key_3(&mut key_reverse, col)
        }

        if key_forward < key_reverse {
            return key_forward / 3;
        } else {
            return key_reverse / 3;
        }
    }

    pub fn possible_non_losing_moves(&self) -> u64 {
        let mut possible_mask: u64 = self.possible();
        let opponent_win: u64 = self.opponent_winning_positions();
        let forced_moves: u64 = possible_mask & opponent_win;

        if forced_moves != 0 {
            if forced_moves.count_ones() > 1 {
                return 0;
            } else {
                possible_mask = forced_moves;
            }
        }
        return possible_mask & !(opponent_win >> 1);
    }

    pub fn move_score(&self, m: u64) -> u32 {
        return (self.compute_winning_positions(&self.current_position | m, self.mask))
            .count_ones();
    }

    pub fn can_play(&self, col: i32) -> bool {
        return (self.mask & Position::top_mask_col(col)) == 0;
    }

    pub fn is_winning_move(&self, col: i32) -> bool {
        return self.winning_positions() & self.possible() & Position::column_mask(col) != 0;
    }

    fn partial_key_3(&self, key: &mut u64, col: i32) {
        let mut pos: u64 = 1 << (col * (Position::HEIGHT + 1));

        while pos & self.mask != 0 {
            *key *= 3;
            if pos & self.current_position != 0 {
                *key += 1
            } else {
                *key += 2
            }
            pos <<= 1;
        }
        *key *= 3;
    }

    fn winning_positions(&self) -> u64 {
        return self.compute_winning_positions(self.current_position, self.mask);
    }

    fn opponent_winning_positions(&self) -> u64 {
        return self.compute_winning_positions(self.current_position ^ self.mask, self.mask);
    }

    fn possible(&self) -> u64 {
        return (self.mask + Position::BOTTOM_MASK) & Position::BOARD_MASK;
    }

    pub fn compute_winning_positions(&self, position: u64, mask: u64) -> u64 {
        // Vertical
        let mut r: u64 = (position << 1) & (position << 2) & (position << 3);

        // Horizontal
        let mut p: u64 =
            (position << (Position::HEIGHT + 1)) & (position << 2 * (Position::HEIGHT + 1));
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

        return r & (Position::BOARD_MASK ^ mask);
    }

    fn top_mask_col(col: i32) -> u64 {
        return 1 << ((Position::HEIGHT - 1) + col * (Position::HEIGHT + 1));
    }

    fn bottom_mask_col(col: i32) -> u64 {
        return 1 << col * (Position::HEIGHT + 1);
    }

    pub fn column_mask(col: i32) -> u64 {
        return ((1 << Position::HEIGHT) - 1) << col * (Position::HEIGHT + 1);
    }

    pub fn disk_to_play(&self) -> Disk {
        match self.nb_moves() % 2 {
            0 => Disk::X,
            1 => Disk::O,
            _ => unreachable!(),
        }
    }

    pub fn playable_row_in_col(&self, col: i32) -> Option<i32> {
        for i in 0..Position::HEIGHT {
            if self.mask & 1 << (col * (Position::HEIGHT + 1) + i) == 0 {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn get_space(&self, col: i32, row: i32) -> Option<Disk> {
        let mask = 1 << (col * 7 + row);

        if self.current_position & mask != 0 {
            return Some(if self.moves % 2 == 0 {
                Disk::X
            } else {
                Disk::O
            });
        } else if self.mask & mask != 0 {
            return Some(if self.moves % 2 == 0 {
                Disk::O
            } else {
                Disk::X
            });
        } else {
            return None;
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for row in (0..Position::HEIGHT).rev() {
            s.push(char::from_u32((row + 49) as u32).unwrap());
            s.push(' ');
            for col in 0..Position::WIDTH {
                s.push_str({
                    match self.get_space(col, row) {
                        Some(Disk::X) => "X ",
                        Some(Disk::O) => "O ",
                        None => ". ",
                    }
                })
            }
            s.push('\n');
        }

        s.push_str("  ");
        for col in 0..Position::WIDTH {
            s.push(char::from_u32((col + 65) as u32).unwrap());
            s.push(' ');
        }
        return s;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

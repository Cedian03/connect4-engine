use crate::position::Position;

pub struct MoveSorter {
    size: u32,
    entries: [Entry; Position::WIDTH as usize],
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    m: u64,
    s: u32,
}

impl MoveSorter {
    pub fn new() -> Self {
        return MoveSorter {
            size: 0,
            entries: [Entry { m: 0, s: 0 }; Position::WIDTH as usize],
        };
    }

    pub fn add(&mut self, m: u64, s: u32) {
        let mut pos = self.size as usize;
        while pos != 0 && self.entries[pos - 1].s > s {
            self.entries[pos] = self.entries[pos - 1];
            pos -= 1;
        }
        self.entries[pos].m = m;
        self.entries[pos].s = s;
        self.size += 1;
    }

    pub fn get_next(&mut self) -> Option<u64> {
        if self.size != 0 {
            self.size -= 1;
            return Some(self.entries[self.size as usize].m);
        } else {
            return None;
        }
    }

    pub fn reset(&mut self) {
        self.size = 0;
    }
}

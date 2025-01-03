use crate::prelude::*;

#[derive(Debug, Default)]
pub struct MoveSorter {
    size: usize,
    entries: [Entry; Position::WIDTH],
}

#[derive(Clone, Debug, Default)]
struct Entry {
    pub mask: BitMask,
    pub score: u32,
}

impl MoveSorter {
    pub fn add(&mut self, mask: BitMask, score: u32) {
        let mut pos = self.size;
        while pos != 0 && self.entries[pos - 1].score > score {
            self.entries.swap(pos, pos - 1);
            pos -= 1;
        }
        self.entries[pos] = Entry { mask, score };
        self.size += 1;
    }

    pub fn get_next(&mut self) -> Option<BitMask> {
        (self.size != 0).then(|| {
            self.size -= 1;
            self.entries[self.size].mask
        })
    }
}

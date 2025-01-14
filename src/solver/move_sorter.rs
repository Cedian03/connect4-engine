use crate::{bit_board::BitBoard, bit_mask, magic::*};

#[derive(Clone)]
pub struct MoveSorter<const W: usize, const H: usize>
where
    BitBoard<W, H>: AsBitMask,
{
    len: usize,
    entries: [Entry<W, H>; W],
}

impl<const W: usize, const H: usize> Default for MoveSorter<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    fn default() -> Self {
        Self {
            len: 0,
            entries: [Default::default(); W],
        }
    }
}

#[derive(Copy, Clone)]
struct Entry<const W: usize, const H: usize>
where
    BitBoard<W, H>: AsBitMask,
{
    mask: bit_mask!(W, H),
    score: u32,
}

impl<const W: usize, const H: usize> Default for Entry<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    fn default() -> Self {
        Self {
            mask: 0.into(),
            score: 0,
        }
    }
}

impl<const W: usize, const H: usize> MoveSorter<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    pub fn add(&mut self, mask: bit_mask!(W, H), score: u32) {
        let mut pos = self.len;
        while pos != 0 && self.entries[pos - 1].score > score {
            self.entries.swap(pos, pos - 1);
            pos -= 1;
        }
        self.entries[pos] = Entry { mask, score };
        self.len += 1;
    }
}

impl<const W: usize, const H: usize> Iterator for MoveSorter<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    type Item = bit_mask!(W, H);

    fn next(&mut self) -> Option<Self::Item> {
        (self.len > 0).then(|| {
            self.len -= 1;
            self.entries[self.len].mask
        })
    }
}

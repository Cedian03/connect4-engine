use num_traits::Zero;

use crate::{
    bit_mask,
    magic::{Bar, BitMask, Foo},
};

pub struct MoveSorter<const W: usize, const H: usize>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
{
    len: usize,
    entries: [Entry<W, H>; W],
}

impl<const W: usize, const H: usize> Default for MoveSorter<W, H>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
{
    fn default() -> Self {
        Self {
            len: Zero::zero(),
            entries: [Default::default(); W],
        }
    }
}

#[derive(Copy, Clone)]
struct Entry<const W: usize, const H: usize>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
{
    mask: bit_mask!(W, H),
    score: u32,
}

impl<const W: usize, const H: usize> Default for Entry<W, H>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
{
    fn default() -> Self {
        Self {
            mask: Zero::zero(),
            score: 0,
        }
    }
}

impl<const W: usize, const H: usize> MoveSorter<W, H>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
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
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask,
{
    type Item = bit_mask!(W, H);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.entries[self.len].mask)
        } else {
            None
        }
    }
}

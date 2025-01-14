use num_traits::AsPrimitive;

use crate::{bit_board::BitBoard, bit_mask, magic::*};

#[derive(Debug)]
pub struct TranspositionTable<const W: usize, const H: usize> {
    keys: Vec<u32>,
    vals: Vec<i8>,
    size: usize,
}

impl<const W: usize, const H: usize> TranspositionTable<W, H> {
    pub fn new(log_size: usize) -> Self {
        let size = table_size(log_size);

        Self {
            keys: vec![0; size],
            vals: vec![0; size],
            size,
        }
    }

    pub fn from_parts(keys: Vec<u32>, vals: Vec<i8>) -> Self {
        assert!(keys.len() == vals.len());

        Self {
            size: keys.len(),
            keys,
            vals,
        }
    }

    pub fn clear(&mut self) {
        self.keys.iter_mut().for_each(|x| *x = 0);
        self.vals.iter_mut().for_each(|x| *x = 0);
    }
}

impl<const W: usize, const H: usize> TranspositionTable<W, H>
where
    BitBoard<W, H>: AsBitMask,
{
    pub fn get(&self, key: bit_mask!(W, H)) -> Option<i32> {
        let i = self.index(key);
        (self.keys[i] == key.as_())
            .then(|| self.vals[i] as i32)
            .filter(|&v| v != 0)
    }

    pub fn put(&mut self, key: bit_mask!(W, H), val: i8) {
        let i = self.index(key);
        self.keys[i] = key.as_();
        self.vals[i] = val;
    }

    fn index(&self, key: bit_mask!(W, H)) -> usize {
        return <bit_mask!(W, H) as AsPrimitive<usize>>::as_(key) % self.size;
    }
}

pub fn table_size(log_size: usize) -> usize {
    next_prime(1 << log_size)
}

fn next_prime(n: usize) -> usize {
    if has_factor(n, 2, n) {
        return next_prime(n + 1);
    } else {
        return n;
    }
}

fn has_factor(n: usize, min: usize, max: usize) -> bool {
    if min * min > n {
        return false;
    } else {
        if min + 1 >= max {
            return n % min == 0;
        } else {
            return has_factor(n, min, med(min, max)) || has_factor(n, med(min, max), max);
        }
    }
}

fn med(min: usize, max: usize) -> usize {
    return min + (max - min) / 2;
}

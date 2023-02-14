extern crate num;

use num::Integer;

const fn med(min: u64, max: u64) -> u64 {
    return (min + max) / 2;
}

const fn has_factor(n: u64, min: u64, max: u64) -> bool {
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

pub const fn next_prime(n: u64) -> u64 {
    if has_factor(n, 2, n) {
        return next_prime(n + 1);
    } else {
        return n;
    }
}

fn log2(n: u64) -> u32 {
    if n <= 1 {
        return 0;
    } else {
        return log2(n / 2) + 1;
    }
}

#[derive(Debug)]
pub struct TranspositionTable<T: Integer> {
    keys: Vec<T>,
    vals: Vec<i8>,
    size: usize,
}

impl<T: Integer> TranspositionTable<T> {
    const fn index(&self, key: u64) -> usize {
        return (key as usize) % self.size;
    }

    pub fn reset(&mut self) {
        todo!()
    }

    pub fn get_mut_keys(&mut self) -> &mut Vec<T> {
        return &mut self.keys;
    }

    pub fn get_mut_values(&mut self) -> &mut Vec<i8> {
        return &mut self.vals;
    }

    pub fn get_mut_size(&mut self) -> &mut usize {
        return &mut self.size;
    }
}

pub trait TableConstructor {
    fn new(table_size: u8) -> Self;
}

impl TableConstructor for TranspositionTable<u32> {
    fn new(table_size: u8) -> Self {
        let size = next_prime(1 << table_size) as usize;
        return TranspositionTable {
            keys: vec![0; size],
            vals: vec![0; size],
            size,
        };
    }
}

impl TableConstructor for TranspositionTable<u8> {
    fn new(table_size: u8) -> Self {
        let size = next_prime(1 << table_size) as usize;
        return TranspositionTable {
            keys: vec![0; size],
            vals: vec![0; size],
            size,
        };
    }
}

pub trait TableGetter {
    fn put(&mut self, key: u64, val: i8);
    fn get(&self, key: u64) -> Option<i8>;
}

impl TableGetter for TranspositionTable<u32> {
    fn put(&mut self, key: u64, val: i8) {
        let i: usize = self.index(key);
        self.keys[i] = key as u32;
        self.vals[i] = val;
    }

    fn get(&self, key: u64) -> Option<i8> {
        let i: usize = self.index(key);
        if self.keys[i] == key as u32 {
            return Some(self.vals[i]);
        } else {
            return None;
        }
    }
}

impl TableGetter for TranspositionTable<u8> {
    fn put(&mut self, key: u64, val: i8) {
        let i: usize = self.index(key);
        self.keys[i] = key as u8;
        self.vals[i] = val;
    }

    fn get(&self, key: u64) -> Option<i8> {
        let i: usize = self.index(key);
        if self.keys[i] == key as u8 {
            return Some(self.vals[i]);
        } else {
            return None;
        }
    }
}

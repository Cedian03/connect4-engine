extern crate num;
use num::Integer;

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
    return (min + max) / 2;
}

#[derive(Debug)]
pub struct TranspositionTable<T: Integer> {
    keys: Vec<T>,
    vals: Vec<i8>,
    pub size: usize,
}

impl<T: Integer> TranspositionTable<T> {
    fn index(&self, key: u64) -> usize {
        return (key as usize) % self.size;
    }

    pub(crate) fn get_mut_keys(&mut self) -> &mut Vec<T> {
        return &mut self.keys;
    }

    pub(crate) fn get_mut_values(&mut self) -> &mut Vec<i8> {
        return &mut self.vals;
    }

    pub(crate) fn get_mut_size(&mut self) -> &mut usize {
        return &mut self.size;
    }
}

pub trait Table {
    fn new(log_size: usize) -> Self;

    fn put(&mut self, key: u64, val: i8);
    fn get(&self, key: u64) -> Option<i32>;
}

impl Table for TranspositionTable<u32> {
    fn new(log_size: usize) -> Self {
        let size = next_prime(1 << log_size);
        return TranspositionTable {
            keys: vec![0; size],
            vals: vec![0; size],
            size,
        };
    }

    fn put(&mut self, key: u64, val: i8) {
        let i = self.index(key);
        self.keys[i] = key as u32;
        self.vals[i] = val;
    }

    fn get(&self, key: u64) -> Option<i32> {
        let i = self.index(key);
        if self.keys[i] == key as u32 {
            let val = self.vals[i];
            if val != 0 {
                return Some(val as i32);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

impl Table for TranspositionTable<u8> {
    fn new(log_size: usize) -> Self {
        let size = next_prime(1 << log_size);
        return TranspositionTable {
            keys: vec![0; size],
            vals: vec![0; size],
            size,
        };
    }

    fn put(&mut self, key: u64, val: i8) {
        let i = self.index(key);
        self.keys[i] = key as u8;
        self.vals[i] = val;
    }

    fn get(&self, key: u64) -> Option<i32> {
        let i = self.index(key);
        if self.keys[i] == key as u8 {
            return Some(self.vals[i] as i32);
        } else {
            return None;
        }
    }
}

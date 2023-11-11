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
pub struct TranspositionTable {
    keys: Vec<u32>,
    vals: Vec<i8>,
    pub size: usize,
}

impl TranspositionTable {
    pub fn new(log_size: usize) -> Self {
        let size = next_prime(log_size);
        Self { 
            keys: Vec::new(), 
            vals: Vec::new(), 
            size
        }
    }

    pub fn put(&mut self, key: u64, val: i8) {
        let i = self.index(key);
        self.keys[i] = key as u32;
        self.vals[i] = val;
    }

    pub fn get(&self, key: u64) -> Option<i32> {
        let i = self.index(key);
        (self.keys[i] == key as u32).then(|| {
            self.vals[i] as i32
        })
    }

    fn index(&self, key: u64) -> usize {
        return (key as usize) % self.size;
    }

    pub(crate) fn get_mut_keys(&mut self) -> &mut Vec<u32> {
        return &mut self.keys;
    }

    pub(crate) fn get_mut_values(&mut self) -> &mut Vec<i8> {
        return &mut self.vals;
    }
}

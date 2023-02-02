use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::position::Position;
use crate::transposition_table::*;

#[derive(Debug)]
pub struct OpeningBook {
    width: u8,
    height: u8,
    depth: Option<u8>,
    table: TranspositionTable<u8>,
}

impl OpeningBook {
    pub fn new(width: i32, height: i32) -> Self {
        return OpeningBook {
            width: width as u8,
            height: height as u8,
            depth: None,
            table: TranspositionTable::new(0),
        };
    }

    pub fn load(&mut self, path: &str) -> io::Result<()> {
        self.depth = None;
        let mut f = File::open(path).expect("Failed to open file");

        let mut meta_buf = [0u8; 6];
        f.read_exact(&mut meta_buf)
            .expect("Failed to load meta data");

        let _width = meta_buf[0];
        let _height = meta_buf[1];
        let _depth = meta_buf[2];
        let key_size = meta_buf[3] as usize;
        let val_size = meta_buf[4] as usize;
        let log_size = meta_buf[5] as usize;

        assert!(_width == self.width);
        assert!(_height == self.height);
        assert!(_depth <= self.width * self.height);
        assert!(key_size == 1);
        assert!(val_size == 1);
        assert!(log_size <= 40);

        let size = next_prime(1 << log_size as u64) as usize;
        let mut_size = self.table.get_mut_size();
        *mut_size = size;

        let mut keys_buf: Vec<u8> = vec![0u8; size * key_size];
        f.read_exact(&mut keys_buf).expect("Failed to load keys");
        let mut_keys: &mut Vec<u8> = self.table.get_mut_keys();
        *mut_keys = keys_buf.into_iter().map(|x| x as u8).collect();

        let mut vals_buf: Vec<u8> = vec![0u8; size * val_size];
        f.read_exact(&mut vals_buf).expect("Failed to load values");
        let mut_vals: &mut Vec<i8> = self.table.get_mut_values();
        *mut_vals = vals_buf.into_iter().map(|x| x as i8).collect();

        self.depth = Some(_depth);
        return Ok(());
    }

    pub fn get(&self, p: &Position) -> Option<i8> {
        match self.depth {
            Some(depth) => {
                if p.nb_moves() <= depth as i32 {
                    return self.table.get(p.key_3());
                } else {
                    return None;
                }
            }
            None => None,
        }
    }
}

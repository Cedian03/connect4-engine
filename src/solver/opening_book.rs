use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::prelude::*;
use crate::solver::transposition_table::TranspositionTable;

#[derive(Debug)]
pub struct OpeningBook {
    table: TranspositionTable,
    depth: usize,
}

impl OpeningBook {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut meta_buf = [0; 6];
        f.read_exact(&mut meta_buf)?;

        let width = meta_buf[0] as usize;
        let height = meta_buf[1] as usize;
        let depth = meta_buf[2] as usize;
        let key_size = meta_buf[3] as usize;
        let val_size = meta_buf[4] as usize;
        let log_size = meta_buf[5] as usize;

        (width == Position::WIDTH).then(|| ())
            .ok_or(Error::LoadBook("Invaild width".to_string()))?;
        (height == Position::HEIGHT).then(|| ())
            .ok_or(Error::LoadBook("Invalid height".to_string()))?;
        (depth <= Position::AREA as usize).then(|| ())
            .ok_or(Error::LoadBook("Invalid depth".to_string()))?;
        (key_size == 1).then(|| ())
            .ok_or(Error::LoadBook("Invalid key size".to_string()))?;
        (val_size == 1).then(|| ())
            .ok_or(Error::LoadBook("Invalid value size".to_string()))?;
        (log_size <= 40).then(|| ())
            .ok_or(Error::LoadBook("Invalid table size".to_string()))?;

        let mut table = TranspositionTable::new(log_size);

        let mut keys_buf = vec![0; table.size * key_size];
        f.read_exact(&mut keys_buf)?;
        let mut_keys = table.get_mut_keys();
        *mut_keys = keys_buf.into_iter().map(|x| x as u32).collect();

        let mut vals_buf = vec![0; table.size * val_size];
        f.read_exact(&mut vals_buf)?;
        let mut_vals = table.get_mut_values();
        *mut_vals = vals_buf.into_iter().map(|x| x as i8).collect();

        Ok(Self { table, depth })
    }

    pub fn get(&self, p: &Position) -> Option<i32> {
        if p.half_turn() <= self.depth as i32 {
            return self.table.get(p.key_3());
        } else {
            return None;
        }
    }
}

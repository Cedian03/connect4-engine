use std::fs::File;
use std::io::Read;
use std::path::Path;

use num_traits::AsPrimitive;

use super::TranspositionTable;
use crate::{
    magic::{Bar, BitMask, Foo},
    Error, Position, Result,
};

#[derive(Debug)]
pub struct OpeningBook<const W: usize, const H: usize> {
    table: TranspositionTable<W, H>,
    depth: usize,
}

impl<const W: usize, const H: usize> OpeningBook<W, H>
where
    Foo<W, H>: Bar,
    <Foo<W, H> as Bar>::Qux: BitMask + AsPrimitive<u32> + AsPrimitive<usize>,
{
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut meta_buf = [0; 6];
        f.read_exact(&mut meta_buf)?;

        let width = meta_buf[0] as usize;
        let height = meta_buf[1] as usize;
        let depth = meta_buf[2] as usize;
        let key_size = meta_buf[3] as usize;
        let val_size = meta_buf[4] as usize;
        let log_size = meta_buf[5] as usize;

        (width == W)
            .then(|| ())
            .ok_or(Error::LoadBook("Invaild width".to_string()))?;
        (height == H)
            .then(|| ())
            .ok_or(Error::LoadBook("Invalid height".to_string()))?;
        (depth <= W * H)
            .then(|| ())
            .ok_or(Error::LoadBook("Invalid depth".to_string()))?;
        (key_size == 1)
            .then(|| ())
            .ok_or(Error::LoadBook("Invalid key size".to_string()))?;
        (val_size == 1)
            .then(|| ())
            .ok_or(Error::LoadBook("Invalid value size".to_string()))?;
        (log_size <= 40)
            .then(|| ())
            .ok_or(Error::LoadBook("Invalid table size".to_string()))?;

        let size = TranspositionTable::size(log_size);

        let key_bytes = key_size * size;
        let val_bytes = val_size * size;

        let mut buf = vec![0; usize::max(key_bytes, val_bytes)];

        let key_view = &mut buf[0..key_bytes];
        f.read_exact(key_view)?;
        let keys = key_view.iter().map(|x| *x as u32).collect();

        let val_view = &mut buf[0..val_bytes];
        f.read_exact(val_view)?;
        let vals = val_view.iter().map(|x| *x as i8).collect();

        Ok(Self {
            table: TranspositionTable::from_parts(keys, vals),
            depth,
        })
    }

    pub fn get(&self, p: &Position<W, H>) -> Option<i32> {
        if p.half_turn() <= self.depth as i32 {
            return self.table.get(p.key_3());
        } else {
            return None;
        }
    }
}

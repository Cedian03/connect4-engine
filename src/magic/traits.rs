use std::ops;

use num_traits::{AsPrimitive, NumAssign, PrimInt};

use crate::board::Board;

pub type BitMask<const W: usize, const H: usize> = <Board<W, H> as AsBitBoard>::BitMask;

pub trait AsBitBoard {
    type BitMask: BitMaskOps;
}

pub trait BitMaskOps:
    NumAssign
    + PrimInt
    + From<u8>
    + AsPrimitive<u8>
    + AsPrimitive<u16>
    + AsPrimitive<u32>
    + AsPrimitive<u64>
    + AsPrimitive<u128>
    + AsPrimitive<usize>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign
    + ops::ShlAssign<usize>
    + ops::ShrAssign
    + ops::ShrAssign<usize>
{
}

impl BitMaskOps for u8 {}
impl BitMaskOps for u16 {}
impl BitMaskOps for u32 {}
impl BitMaskOps for u64 {}
impl BitMaskOps for u128 {}

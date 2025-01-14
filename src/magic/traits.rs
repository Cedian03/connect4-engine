use std::ops;

use num_traits::{AsPrimitive, NumAssign, PrimInt};

#[macro_export]
macro_rules! bit_mask {
    ($w:ident, $h:ident) => {
        <$crate::board::BitBoard<$w, $h> as $crate::magic::AsBitMask>::BitMask
    };
}

pub trait AsBitMask
where
    Self::BitMask: BitMaskOps,
{
    type BitMask;
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

//! Not really magic is it...

use std::ops;

use num_traits::{NumAssign, PrimInt};

#[macro_export]
macro_rules! bit_mask {
    ($w:ident, $h:ident) => {
        <crate::magic::Foo<$w, $h> as crate::magic::Bar>::Qux
    };
}

pub trait BitMask:
    NumAssign
    + PrimInt
    + From<u8>
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign
    + ops::ShlAssign<usize>
    + ops::ShrAssign
    + ops::ShrAssign<usize>
{
}

impl<T> BitMask for T where
    T: NumAssign
        + PrimInt
        + From<u8>
        + ops::BitAndAssign
        + ops::BitOrAssign
        + ops::BitXorAssign
        + ops::ShlAssign
        + ops::ShlAssign<usize>
        + ops::ShrAssign
        + ops::ShrAssign<usize>
{
}

pub struct Foo<const W: usize, const H: usize>;

pub trait Bar {
    type Qux;
}

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

impl Bar for Foo<4, 4> {
    type Qux = u32;
}

impl Bar for Foo<4, 5> {
    type Qux = u32;
}

impl Bar for Foo<4, 6> {
    type Qux = u32;
}

impl Bar for Foo<4, 7> {
    type Qux = u32;
}

impl Bar for Foo<4, 8> {
    type Qux = u64;
}

impl Bar for Foo<4, 9> {
    type Qux = u64;
}

impl Bar for Foo<4, 10> {
    type Qux = u64;
}

impl Bar for Foo<4, 11> {
    type Qux = u64;
}

impl Bar for Foo<4, 12> {
    type Qux = u64;
}

impl Bar for Foo<4, 13> {
    type Qux = u64;
}

impl Bar for Foo<4, 14> {
    type Qux = u64;
}

impl Bar for Foo<4, 15> {
    type Qux = u64;
}

impl Bar for Foo<5, 4> {
    type Qux = u32;
}

impl Bar for Foo<5, 5> {
    type Qux = u32;
}

impl Bar for Foo<5, 6> {
    type Qux = u64;
}

impl Bar for Foo<5, 7> {
    type Qux = u64;
}

impl Bar for Foo<5, 8> {
    type Qux = u64;
}

impl Bar for Foo<5, 9> {
    type Qux = u64;
}

impl Bar for Foo<5, 10> {
    type Qux = u64;
}

impl Bar for Foo<5, 11> {
    type Qux = u64;
}

impl Bar for Foo<5, 12> {
    type Qux = u128;
}

impl Bar for Foo<5, 13> {
    type Qux = u128;
}

impl Bar for Foo<5, 14> {
    type Qux = u128;
}

impl Bar for Foo<5, 15> {
    type Qux = u128;
}

impl Bar for Foo<6, 4> {
    type Qux = u32;
}

impl Bar for Foo<6, 5> {
    type Qux = u64;
}

impl Bar for Foo<6, 6> {
    type Qux = u64;
}

impl Bar for Foo<6, 7> {
    type Qux = u64;
}

impl Bar for Foo<6, 8> {
    type Qux = u64;
}

impl Bar for Foo<6, 9> {
    type Qux = u64;
}

impl Bar for Foo<6, 10> {
    type Qux = u128;
}

impl Bar for Foo<6, 11> {
    type Qux = u128;
}

impl Bar for Foo<6, 12> {
    type Qux = u128;
}

impl Bar for Foo<6, 13> {
    type Qux = u128;
}

impl Bar for Foo<6, 14> {
    type Qux = u128;
}

impl Bar for Foo<6, 15> {
    type Qux = u128;
}

impl Bar for Foo<7, 4> {
    type Qux = u64;
}

impl Bar for Foo<7, 5> {
    type Qux = u64;
}

impl Bar for Foo<7, 6> {
    type Qux = u64;
}

impl Bar for Foo<7, 7> {
    type Qux = u64;
}

impl Bar for Foo<7, 8> {
    type Qux = u64;
}

impl Bar for Foo<7, 9> {
    type Qux = u128;
}

impl Bar for Foo<7, 10> {
    type Qux = u128;
}

impl Bar for Foo<7, 11> {
    type Qux = u128;
}

impl Bar for Foo<7, 12> {
    type Qux = u128;
}

impl Bar for Foo<7, 13> {
    type Qux = u128;
}

impl Bar for Foo<7, 14> {
    type Qux = u128;
}

impl Bar for Foo<7, 15> {
    type Qux = u128;
}

impl Bar for Foo<8, 4> {
    type Qux = u64;
}

impl Bar for Foo<8, 5> {
    type Qux = u64;
}

impl Bar for Foo<8, 6> {
    type Qux = u64;
}

impl Bar for Foo<8, 7> {
    type Qux = u64;
}

impl Bar for Foo<8, 8> {
    type Qux = u128;
}

impl Bar for Foo<8, 9> {
    type Qux = u128;
}

impl Bar for Foo<8, 10> {
    type Qux = u128;
}

impl Bar for Foo<8, 11> {
    type Qux = u128;
}

impl Bar for Foo<8, 12> {
    type Qux = u128;
}

impl Bar for Foo<8, 13> {
    type Qux = u128;
}

impl Bar for Foo<8, 14> {
    type Qux = u128;
}

impl Bar for Foo<8, 15> {
    type Qux = u128;
}

impl Bar for Foo<9, 4> {
    type Qux = u64;
}

impl Bar for Foo<9, 5> {
    type Qux = u64;
}

impl Bar for Foo<9, 6> {
    type Qux = u64;
}

impl Bar for Foo<9, 7> {
    type Qux = u128;
}

impl Bar for Foo<9, 8> {
    type Qux = u128;
}

impl Bar for Foo<9, 9> {
    type Qux = u128;
}

impl Bar for Foo<9, 10> {
    type Qux = u128;
}

impl Bar for Foo<9, 11> {
    type Qux = u128;
}

impl Bar for Foo<9, 12> {
    type Qux = u128;
}

impl Bar for Foo<9, 13> {
    type Qux = u128;
}

impl Bar for Foo<10, 4> {
    type Qux = u64;
}

impl Bar for Foo<10, 5> {
    type Qux = u64;
}

impl Bar for Foo<10, 6> {
    type Qux = u128;
}

impl Bar for Foo<10, 7> {
    type Qux = u128;
}

impl Bar for Foo<10, 8> {
    type Qux = u128;
}

impl Bar for Foo<10, 9> {
    type Qux = u128;
}

impl Bar for Foo<10, 10> {
    type Qux = u128;
}

impl Bar for Foo<10, 11> {
    type Qux = u128;
}

impl Bar for Foo<11, 4> {
    type Qux = u64;
}

impl Bar for Foo<11, 5> {
    type Qux = u128;
}

impl Bar for Foo<11, 6> {
    type Qux = u128;
}

impl Bar for Foo<11, 7> {
    type Qux = u128;
}

impl Bar for Foo<11, 8> {
    type Qux = u128;
}

impl Bar for Foo<11, 9> {
    type Qux = u128;
}

impl Bar for Foo<11, 10> {
    type Qux = u128;
}

impl Bar for Foo<12, 4> {
    type Qux = u64;
}

impl Bar for Foo<12, 5> {
    type Qux = u128;
}

impl Bar for Foo<12, 6> {
    type Qux = u128;
}

impl Bar for Foo<12, 7> {
    type Qux = u128;
}

impl Bar for Foo<12, 8> {
    type Qux = u128;
}

impl Bar for Foo<12, 9> {
    type Qux = u128;
}

impl Bar for Foo<13, 4> {
    type Qux = u128;
}

impl Bar for Foo<13, 5> {
    type Qux = u128;
}

impl Bar for Foo<13, 6> {
    type Qux = u128;
}

impl Bar for Foo<13, 7> {
    type Qux = u128;
}

impl Bar for Foo<13, 8> {
    type Qux = u128;
}

impl Bar for Foo<14, 4> {
    type Qux = u128;
}

impl Bar for Foo<14, 5> {
    type Qux = u128;
}

impl Bar for Foo<14, 6> {
    type Qux = u128;
}

impl Bar for Foo<14, 7> {
    type Qux = u128;
}

impl Bar for Foo<14, 8> {
    type Qux = u128;
}

impl Bar for Foo<15, 4> {
    type Qux = u128;
}

impl Bar for Foo<15, 5> {
    type Qux = u128;
}

impl Bar for Foo<15, 6> {
    type Qux = u128;
}

impl Bar for Foo<15, 7> {
    type Qux = u128;
}


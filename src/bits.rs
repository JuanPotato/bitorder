use core::cmp::{Eq, PartialEq};
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

// Not all traits integer types satisfy, but enough for what I'm doing
pub trait Uint
where
    Self: Sized
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitOr<Output = Self>
        + BitOrAssign
        + Not<Output = Self>
        + Shl<usize, Output = Self>
        + Shr<usize, Output = Self>
        + From<u8>
        + TryInto<u8>
        + PartialEq
        + Eq
        + Copy,
{
    const WIDTH: usize;
    const ONE: Self;
    const ZERO: Self;

    /// Returns the bits from [start, stop) shifted so that start becomes 0
    fn get_bit_slice(self, start: usize, stop: usize) -> Self {
        let len = stop - start;
        let bitmask = ((!Self::ZERO) >> (Self::WIDTH - len)) << start;
        (self & bitmask) >> start
    }

    /// Copies the bits [start, stop) from val to self and returns the result
    fn set_bit_slice(self, start: usize, stop: usize, val: Self) -> Self {
        let len = stop - start;
        let bitmask = ((!Self::ZERO) >> (Self::WIDTH - len)) << start;
        (self & !bitmask) | ((val << start) & bitmask)
    }
}

macro_rules! impl_uint_trait {
    ($num_type:ident, $bit_count:expr) => {
        impl Uint for $num_type {
            const WIDTH: usize = $bit_count;
            const ZERO: $num_type = 0;
            const ONE: $num_type = 1;
        }
    };
}

impl_uint_trait!(u8, 8);
impl_uint_trait!(u16, 16);
impl_uint_trait!(u32, 32);
impl_uint_trait!(u64, 64);
impl_uint_trait!(u128, 128);

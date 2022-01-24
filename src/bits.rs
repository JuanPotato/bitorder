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

    // Shl that clamps to 0 if rhs is out of range
    fn saturating_shl(self, rhs: usize) -> Self;

    // Shr that clamps to 0 if rhs is out of range
    fn saturating_shr(self, rhs: usize) -> Self;

    fn as_u8(self) -> u8;
}

macro_rules! impl_uint_trait {
    ($num_type:ident) => {
        impl Uint for $num_type {
            const WIDTH: usize = $num_type::BITS as usize;
            const ZERO: $num_type = 0;
            const ONE: $num_type = 1;

            #[inline(always)]
            fn saturating_shl(self, rhs: usize) -> Self {
                self.checked_shl(rhs as u32).unwrap_or(0)
            }

            #[inline(always)]
            fn saturating_shr(self, rhs: usize) -> Self {
                self.checked_shr(rhs as u32).unwrap_or(0)
            }

            #[inline(always)]
            fn as_u8(self) -> u8 {
                (self & 0xff) as u8
            }
        }
    };
}

impl_uint_trait!(u8);
impl_uint_trait!(u16);
impl_uint_trait!(u32);
impl_uint_trait!(u64);
impl_uint_trait!(u128);

/*!
A small library to read and write bits in Lsb or Msb order.

# Example
```
use bitorder::container::{BitReader, BitWriter};
use bitorder::{Msb0, BitOrder};

let mut data = vec![0u8; 2];
let mut b = BitWriter::<Msb0>::new(&mut data);
b.write_bits(1, 1u8);
b.write_bits(2, 2u8);
b.write_bits(3, 3u8);
b.write_bits(4, 4u8);
b.write_bits(5, 5u8);

assert_eq!(&data, b"\xCD\x0A"); // http://mjfrazer.org/mjfrazer/bitfields/

let mut b = BitReader::<Msb0>::new(&data);
assert_eq!(b.read_bits::<u8>(1), 1u8);
assert_eq!(b.read_bits::<u8>(2), 2u8);
assert_eq!(b.read_bits::<u8>(3), 3u8);
assert_eq!(b.read_bits::<u8>(4), 4u8);
assert_eq!(b.read_bits::<u8>(5), 5u8);
```
 */

mod bits;
pub mod container;

use crate::bits::Uint;

#[derive(Debug, Copy, Clone)]
pub enum Lsb0 {}

#[derive(Debug, Copy, Clone)]
pub enum Msb0 {}

pub trait BitOrder {
    fn read_bits<T: Uint>(data: &[u8], byte_index: usize, bit_index: usize, len: usize) -> T;
    fn write_bits<T: Uint>(
        data: &mut [u8],
        byte_index: usize,
        bit_index: usize,
        len: usize,
        bits: T,
    );
}

impl BitOrder for Lsb0 {
    fn read_bits<T: Uint>(
        data: &[u8],
        mut byte_index: usize,
        mut bit_index: usize,
        mut len: usize,
    ) -> T {
        let mut bits = T::ZERO;
        let mut prev_chunk_size = 0;

        while len != 0 {
            // Read chunk
            let chunk_size = len.min(8 - bit_index).min(8);

            let start = bit_index;
            let stop = start + chunk_size;

            let chunk = data[byte_index].get_bit_slice(start, stop);

            // Advance
            bit_index += chunk_size;
            byte_index += bit_index / 8;
            bit_index %= 8;

            // Add to bits
            let chunk: T = chunk.into();
            bits |= chunk << prev_chunk_size;
            len -= chunk_size;

            prev_chunk_size += chunk_size;
        }

        bits
    }

    fn write_bits<T: Uint>(
        data: &mut [u8],
        mut byte_index: usize,
        mut bit_index: usize,
        mut len: usize,
        mut bits: T,
    ) {
        while len != 0 {
            // Write chunk
            let chunk = bits.as_u8();
            // let chunk = bits.get_bit_slice(0, 8).as_u8();
            let chunk_size = len.min(8 - bit_index);

            let start = bit_index;
            let stop = start + chunk_size;

            data[byte_index] = data[byte_index].set_bit_slice(start, stop, chunk);

            // Advance
            bit_index += chunk_size;
            byte_index += bit_index / 8;
            bit_index %= 8;

            // Remove lower bits
            bits = bits.saturating_shr(chunk_size);
            len -= chunk_size;
        }
    }
}

impl BitOrder for Msb0 {
    fn read_bits<T: Uint>(
        data: &[u8],
        mut byte_index: usize,
        mut bit_index: usize,
        mut len: usize,
    ) -> T {
        let mut bits = T::ZERO;

        while len != 0 {
            // Read chunk
            let chunk_size = len.min(8 - bit_index);

            let stop = 7 - bit_index + 1;
            let start = stop - chunk_size;

            let chunk = data[byte_index].get_bit_slice(start, stop);

            // Advance
            bit_index += chunk_size;
            byte_index += bit_index / 8;
            bit_index %= 8;

            // Add to bits
            bits = (bits.saturating_shl(chunk_size)) | chunk.into();
            len -= chunk_size;
        }

        bits
    }

    fn write_bits<T: Uint>(
        data: &mut [u8],
        mut byte_index: usize,
        mut bit_index: usize,
        mut len: usize,
        bits: T,
    ) {
        while len != 0 {
            // Write chunk
            let chunk_size = len.min(8 - bit_index);
            let chunk = bits.get_bit_slice(len - chunk_size, len).as_u8();

            let stop = 8 - bit_index;
            let start = stop - chunk_size;

            data[byte_index] = data[byte_index].set_bit_slice(start, stop, chunk);

            // Advance
            bit_index += chunk_size;
            byte_index += bit_index / 8;
            bit_index %= 8;

            // Remove top bits
            len -= chunk_size;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::container::{BitReader, BitWriter};
    use crate::{BitOrder, Lsb0, Msb0};

    macro_rules! make_test {
        ($test_name:ident, $num_type:ty, $bits:literal, $value:literal) => {
            #[test]
            fn $test_name() {
                let mut output = vec![0u8; 128];
                for i in 0..=16 {
                    Lsb0::write_bits::<$num_type>(&mut output, i / 8, i % 8, $bits, $value);
                    assert_eq!(
                        Lsb0::read_bits::<$num_type>(&output, i / 8, i % 8, $bits),
                        $value
                    );

                    output.clear();
                    output.resize(128, 0);

                    Msb0::write_bits::<$num_type>(&mut output, i / 8, i % 8, $bits, $value);
                    assert_eq!(
                        Msb0::read_bits::<$num_type>(&output, i / 8, i % 8, $bits),
                        $value
                    );
                }
            }
        };
    }

    make_test!(b8, u8, 8, 0xABu8);
    make_test!(b16, u16, 16, 0xCAFEu16);
    make_test!(b32, u32, 32, 0xDEADBEEFu32);
    make_test!(b64, u64, 64, 0x31415926_53589793u64);

    #[test]
    fn msb_read_works() {
        let mut b = BitReader::<Msb0>::new(b"\xCD\x0A");
        assert_eq!(b.read_bits::<u8>(1), 1u8);
        assert_eq!(b.read_bits::<u16>(2), 2u16);
        assert_eq!(b.read_bits::<u32>(3), 3u32);
        assert_eq!(b.read_bits::<u64>(4), 4u64);
        assert_eq!(b.read_bits::<u128>(5), 5u128);
    }

    #[test]
    fn msb_write_works() {
        let mut data = vec![0u8; 2];
        let mut b = BitWriter::<Msb0>::new(&mut data);
        b.write_bits(1, 1u8);
        b.write_bits(2, 2u8);
        b.write_bits(3, 3u8);
        b.write_bits(4, 4u8);
        b.write_bits(5, 5u8);

        assert_eq!(&data, b"\xCD\x0A");
    }

    #[test]
    fn msb_stress() {
        let mut output = vec![0u8; 128];
        let mut b = BitWriter::<Msb0>::new(&mut output);
        for i in 1..=32 {
            b.write_bits(i, i as u32);
        }

        let mut b = BitReader::<Msb0>::new(&output);
        for i in 1..=32 {
            assert_eq!(b.read_bits::<u32>(i), i as u32);
        }
    }

    #[test]
    fn lsb_read_works() {
        let mut b = BitReader::<Lsb0>::new(b"\x1D\x15");
        assert_eq!(b.read_bits::<u8>(1), 1u8);
        assert_eq!(b.read_bits::<u16>(2), 2u16);
        assert_eq!(b.read_bits::<u32>(3), 3u32);
        assert_eq!(b.read_bits::<u64>(4), 4u64);
        assert_eq!(b.read_bits::<u128>(5), 5u128);
    }

    #[test]
    fn lsb_write_works() {
        let mut data = vec![0u8; 2];
        let mut b = BitWriter::<Lsb0>::new(&mut data);
        b.write_bits(1, 1u8);
        b.write_bits(2, 2u8);
        b.write_bits(3, 3u8);
        b.write_bits(4, 4u8);
        b.write_bits(5, 5u8);

        assert_eq!(&data, b"\x1D\x15");
    }

    #[test]
    fn lsb_stress() {
        let mut output = vec![0u8; 128];
        let mut b = BitWriter::<Lsb0>::new(&mut output);
        for i in 1..=32 {
            b.write_bits(i, i as u32);
        }

        let mut b = BitReader::<Lsb0>::new(&output);
        for i in 1..=32 {
            assert_eq!(b.read_bits::<u32>(i), i as u32);
        }
    }
}

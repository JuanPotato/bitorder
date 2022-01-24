/*!
A small library to read and write bits in Lsb or Msb order.

# Example
```
use bitorder::writer::BitWriterMsb;
use bitorder::reader::BitReaderMsb;

let mut data = vec![0u8; 2];
let mut b = BitWriterMsb::new(&mut data);
b.write_bits(1u8, 1).unwrap();
b.write_bits(2u8, 2).unwrap();
b.write_bits(3u8, 3).unwrap();
b.write_bits(4u8, 4).unwrap();
b.write_bits(5u8, 5).unwrap();

assert_eq!(&data, b"\xCD\x0A"); // http://mjfrazer.org/mjfrazer/bitfields/

let mut b = BitReaderMsb::new(&data);
assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
assert_eq!(b.read_bits::<u8>(2), Ok(2u8));
assert_eq!(b.read_bits::<u8>(3), Ok(3u8));
assert_eq!(b.read_bits::<u8>(4), Ok(4u8));
assert_eq!(b.read_bits::<u8>(5), Ok(5u8));
```
*/

mod bits;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::reader::{BitReaderLsb, BitReaderMsb, read_bits_lsb, read_bits_msb};
    use crate::writer::{BitWriterLsb, BitWriterMsb, write_bits_lsb, write_bits_msb};

    macro_rules! make_test {
        ($test_name:ident, $num_type:ty, $bits:literal, $value:literal) => {
            #[test]
            fn $test_name() {
                let mut output = vec![0u8; 128];
                for i in 0..=$bits {
                    write_bits_lsb::<$num_type>(&mut output, i / 8, i % 8, $bits, $value);
                    assert_eq!(read_bits_lsb::<$num_type>(&output, i / 8, i % 8, $bits), $value);

                    output.clear();
                    output.resize(128, 0);

                    write_bits_msb::<$num_type>(&mut output, i / 8, i % 8, $bits, $value);
                    assert_eq!(read_bits_msb::<$num_type>(&output, i / 8, i % 8, $bits), $value);
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
        let mut b = BitReaderMsb::new(b"\xCD\x0A");
        assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
        assert_eq!(b.read_bits::<u16>(2), Ok(2u16));
        assert_eq!(b.read_bits::<u32>(3), Ok(3u32));
        assert_eq!(b.read_bits::<u64>(4), Ok(4u64));
        assert_eq!(b.read_bits::<u128>(5), Ok(5u128));
    }

    #[test]
    fn msb_write_works() {
        let mut data = vec![0u8; 2];
        let mut b = BitWriterMsb::new(&mut data);
        b.write_bits(1u8, 1).unwrap();
        b.write_bits(2u8, 2).unwrap();
        b.write_bits(3u8, 3).unwrap();
        b.write_bits(4u8, 4).unwrap();
        b.write_bits(5u8, 5).unwrap();

        assert_eq!(&data, b"\xCD\x0A");
    }

    #[test]
    fn msb_stress() {
        let mut output = vec![0u8; 128];
        let mut b = BitWriterMsb::new(&mut output);
        for i in 1..=32 {
            b.write_bits(i as u32, i).unwrap();
        }

        let mut b = BitReaderMsb::new(&output);
        for i in 1..=32 {
            assert_eq!(b.read_bits::<u32>(i), Ok(i as u32));
        }
    }

    #[test]
    fn lsb_read_works() {
        let mut b = BitReaderLsb::new(b"\x1D\x15");
        assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
        assert_eq!(b.read_bits::<u16>(2), Ok(2u16));
        assert_eq!(b.read_bits::<u32>(3), Ok(3u32));
        assert_eq!(b.read_bits::<u64>(4), Ok(4u64));
        assert_eq!(b.read_bits::<u128>(5), Ok(5u128));
    }

    #[test]
    fn lsb_write_works() {
        let mut data = vec![0u8; 2];
        let mut b = BitWriterLsb::new(&mut data);
        b.write_bits(1u8, 1).unwrap();
        b.write_bits(2u8, 2).unwrap();
        b.write_bits(3u8, 3).unwrap();
        b.write_bits(4u8, 4).unwrap();
        b.write_bits(5u8, 5).unwrap();

        assert_eq!(&data, b"\x1D\x15");
    }

    #[test]
    fn lsb_stress() {
        let mut output = vec![0u8; 128];
        let mut b = BitWriterLsb::new(&mut output);
        for i in 1..=32 {
            b.write_bits(i as u32, i).unwrap();
        }

        let mut b = BitReaderLsb::new(&output);
        for i in 1..=32 {
            assert_eq!(b.read_bits::<u32>(i), Ok(i as u32));
        }
    }
}

mod bits;
use bits::Uint;

pub struct BitStreamMsb<'a> {
    data: &'a [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitStreamMsb<'a> {
    pub fn new(data: &'a [u8]) -> BitStreamMsb<'a> {
        BitStreamMsb {
            data: data,
            index: 0,
            bit_index: 7,
        }
    }

    fn read(&mut self, bit_len: usize) -> Result<(u8, usize), String> {
        if bit_len == 0 {
            return Err("Read length cannot be zero bits.".into());
        }

        if self.index >= self.data.len() {
            return Err("No more data to read.".into());
        }

        let read_size = bit_len.min(self.bit_index + 1).min(8);

        let stop = self.bit_index + 1;
        let start = stop - read_size;

        let bits = self.data[self.index].get_bit_slice(start, stop);
        self.advance(read_size);

        Ok((bits, read_size))
    }

    pub fn read_bits<T: Uint>(&mut self, mut len: usize) -> Result<T, String> {
        if len > T::WIDTH {
            return Err(format!("Cannot read {} bits into a {} bit wide type", len, T::WIDTH));
        }

        let (chunk, chunk_size) = self.read(len)?;
        let mut bits = chunk.into();
        len -= chunk_size;

        while len != 0 {
            let (chunk, chunk_size) = self.read(len)?;
            bits = (bits << chunk_size) | chunk.into();
            len -= chunk_size;
        }

        Ok(bits)
    }

    fn advance(&mut self, bits: usize) {
        self.bit_index = 7 - self.bit_index;
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
        self.bit_index = 7 - self.bit_index;
    }
}

pub struct BitStreamLsb<'a> {
    data: &'a [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitStreamLsb<'a> {
    pub fn new(data: &'a [u8]) -> BitStreamLsb<'a> {
        BitStreamLsb {
            data: data,
            index: 0,
            bit_index: 0,
        }
    }

    fn read(&mut self, bit_len: usize) -> Result<(u8, usize), String> {
        if bit_len == 0 {
            return Err("Read length cannot be zero bits.".into());
        }

        if self.index >= self.data.len() {
            return Err("No more data to read.".into());
        }

        let read_size = bit_len.min(8 - self.bit_index).min(8);

        let start = self.bit_index;
        let stop = start + read_size;

        let bits = self.data[self.index].get_bit_slice(start, stop);
        self.advance(read_size);

        Ok((bits, read_size))
    }

    pub fn read_bits<T: Uint>(&mut self, mut len: usize) -> Result<T, String> {
        if len > T::WIDTH {
            return Err(format!("Cannot read {} bits into a {} bit wide type", len, T::WIDTH));
        }

        let (chunk, chunk_size) = self.read(len)?;
        let mut bits = chunk.into();
        len -= chunk_size;

        let mut prev_chunk_size = chunk_size;

        while len != 0 {
            let (chunk, chunk_size) = self.read(len)?;
            let chunk: T = chunk.into();
            bits |= chunk << prev_chunk_size;
            len -= chunk_size;

            prev_chunk_size = chunk_size;
        }

        Ok(bits)
    }

    fn advance(&mut self, bits: usize) {
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
    }
}

#[cfg(test)]
mod tests {
    use crate::{BitStreamLsb, BitStreamMsb};

    #[test]
    fn msb_works() {
        let mut b = BitStreamMsb::new(b"\xCD\x0A");
        assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
        assert_eq!(b.read_bits::<u16>(2), Ok(2u16));
        assert_eq!(b.read_bits::<u32>(3), Ok(3u32));
        assert_eq!(b.read_bits::<u64>(4), Ok(4u64));
        assert_eq!(b.read_bits::<u128>(5), Ok(5u128));
    }

    #[test]
    fn lsb_works() {
        let mut b = BitStreamLsb::new(b"\x1D\x15");
        assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
        assert_eq!(b.read_bits::<u16>(2), Ok(2u16));
        assert_eq!(b.read_bits::<u32>(3), Ok(3u32));
        assert_eq!(b.read_bits::<u64>(4), Ok(4u64));
        assert_eq!(b.read_bits::<u128>(5), Ok(5u128));
    }
}
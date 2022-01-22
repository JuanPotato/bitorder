use crate::bits::Uint;

/// Read bits from a &\[u8\] in Msb order
pub struct BitReaderMsb<'a> {
    data: &'a [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitReaderMsb<'a> {
    pub fn new(data: &'a [u8]) -> BitReaderMsb<'a> {
        BitReaderMsb {
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
            return Err(format!(
                "Cannot read {} bits into a {} bit wide type",
                len,
                T::WIDTH
            ));
        }

        let mut bits = T::ZERO;

        while len != 0 {
            let (chunk, chunk_size) = self.read(len)?;
            bits = (bits.saturating_shl(chunk_size)) | chunk.into();
            len -= chunk_size;
        }

        Ok(bits)
    }

    pub fn advance(&mut self, bits: usize) {
        self.bit_index = 7 - self.bit_index;
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
        self.bit_index = 7 - self.bit_index;
    }
}

/// Read bits from a &\[u8\] in Lsb order
pub struct BitReaderLsb<'a> {
    data: &'a [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitReaderLsb<'a> {
    pub fn new(data: &'a [u8]) -> BitReaderLsb<'a> {
        BitReaderLsb {
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
            return Err(format!(
                "Cannot read {} bits into a {} bit wide type",
                len,
                T::WIDTH
            ));
        }

        let mut bits = T::ZERO;
        let mut prev_chunk_size = 0;

        while len != 0 {
            let (chunk, chunk_size) = self.read(len)?;
            let chunk: T = chunk.into();
            bits |= chunk << prev_chunk_size;
            len -= chunk_size;

            prev_chunk_size += chunk_size;
        }

        Ok(bits)
    }

    pub fn advance(&mut self, bits: usize) {
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
    }
}

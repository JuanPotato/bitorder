use crate::bits::Uint;

/// Write bits to a &mut \[u8\] in Msb order
pub struct BitWriterMsb<'a> {
    data: &'a mut [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitWriterMsb<'a> {
    pub fn new(data: &'a mut [u8]) -> BitWriterMsb<'a> {
        BitWriterMsb {
            data: data,
            index: 0,
            bit_index: 7,
        }
    }

    fn write(&mut self, bits: u8, bit_len: usize) -> Result<usize, String> {
        if bit_len == 0 {
            return Err("Write length cannot be zero bits.".into());
        }

        if self.index >= self.data.len() {
            return Err("No more room to write bits.".into());
        }

        let write_size = bit_len.min(self.bit_index + 1).min(8);

        let stop = self.bit_index + 1;
        let start = stop - write_size;
        let offset = bit_len - write_size;

        self.data[self.index] = self.data[self.index].set_bit_slice(start, stop, bits >> offset);
        self.advance(write_size);

        Ok(write_size)
    }

    pub fn write_bits<T: Uint>(&mut self, n: T, mut len: usize) -> Result<(), String> {
        if len > T::WIDTH {
            return Err(format!(
                "Cannot write {} bits from a {} bit wide type",
                len,
                T::WIDTH
            ));
        }

        while len != 0 {
            let chunk_size = len.min(8);
            let chunk = n
                .get_bit_slice(len.saturating_sub(8), len)
                .try_into()
                .ok()
                .unwrap();
            let wrote_size = self.write(chunk, chunk_size)?;
            len -= wrote_size;
        }

        Ok(())
    }

    pub fn advance(&mut self, bits: usize) {
        self.bit_index = 7 - self.bit_index;
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
        self.bit_index = 7 - self.bit_index;
    }
}

/// Write bits to a &mut \[u8\] in Lsb order
pub struct BitWriterLsb<'a> {
    data: &'a mut [u8],
    index: usize,
    bit_index: usize,
}

impl<'a> BitWriterLsb<'a> {
    pub fn new(data: &'a mut [u8]) -> BitWriterLsb<'a> {
        BitWriterLsb {
            data: data,
            index: 0,
            bit_index: 0,
        }
    }

    fn write(&mut self, bits: u8, bit_len: usize) -> Result<usize, String> {
        if bit_len == 0 {
            return Err("Write length cannot be zero bits.".into());
        }

        if self.index >= self.data.len() {
            return Err("No more room to write bits.".into());
        }

        let write_size = bit_len.min(8 - self.bit_index).min(8);

        let start = self.bit_index;
        let stop = start + write_size;

        self.data[self.index] = self.data[self.index].set_bit_slice(start, stop, bits);
        self.advance(write_size);

        Ok(write_size)
    }

    pub fn write_bits<T: Uint>(&mut self, mut n: T, mut len: usize) -> Result<(), String> {
        if len > T::WIDTH {
            return Err(format!(
                "Cannot write {} bits from a {} bit wide type",
                len,
                T::WIDTH
            ));
        }

        while len != 0 {
            let chunk = n.get_bit_slice(0, 8).try_into().ok().unwrap();
            let wrote_size = self.write(chunk, len)?;
            n = n.saturating_shr(wrote_size);
            len -= wrote_size;
        }

        Ok(())
    }

    pub fn advance(&mut self, bits: usize) {
        self.bit_index += bits;
        self.index += self.bit_index / 8;
        self.bit_index %= 8;
    }
}

use crate::bits::Uint;
use crate::BitOrder;
use std::marker::PhantomData;

/// Read bits from a &\[u8\]
pub struct BitReader<'a, B: BitOrder> {
    data: &'a [u8],
    byte_index: usize,
    bit_index: usize,
    _phantom: PhantomData<B>,
}

impl<'a, B: BitOrder> BitReader<'a, B> {
    pub fn new(data: &'a [u8]) -> BitReader<'a, B> {
        BitReader {
            data,
            byte_index: 0,
            bit_index: 0,
            _phantom: PhantomData,
        }
    }

    pub fn read_bits<T: Uint>(&mut self, len: usize) -> T {
        let res = B::read_bits(self.data, self.byte_index, self.bit_index, len);
        self.bit_index += len;
        self.byte_index += self.bit_index / 8;
        self.bit_index %= 8;
        res
    }
}

/// Write bits to a &mut \[u8\]
pub struct BitWriter<'a, B: BitOrder> {
    data: &'a mut [u8],
    byte_index: usize,
    bit_index: usize,
    _phantom: PhantomData<B>,
}

impl<'a, B: BitOrder> BitWriter<'a, B> {
    pub fn new(data: &'a mut [u8]) -> BitWriter<'a, B> {
        BitWriter {
            data,
            byte_index: 0,
            bit_index: 0,
            _phantom: PhantomData,
        }
    }

    pub fn write_bits<T: Uint>(&mut self, len: usize, bits: T) {
        B::write_bits(self.data, self.byte_index, self.bit_index, len, bits);
        self.bit_index += len;
        self.byte_index += self.bit_index / 8;
        self.bit_index %= 8;
    }
}

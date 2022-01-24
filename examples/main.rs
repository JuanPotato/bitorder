#![feature(test)]
extern crate test;

use bitorder::container::{BitReader, BitWriter};
use bitorder::{BitOrder, Lsb0, Msb0};
use std::time::Instant;

use deku::prelude::*;
use test::{black_box, Bencher};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Ipv4Header {
    #[deku(bits = "4")]
    pub version: u8,
    #[deku(bits = "4")]
    pub ihl: u8,
    #[deku(bits = "6")]
    pub dscp: u8,
    #[deku(bits = "2")]
    pub ecn: u8,
    pub length: u16,
    pub identification: u16,
    #[deku(bits = "3")]
    pub flags: u8,
    #[deku(bits = "13")]
    pub offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
}

impl Ipv4Header {
    pub fn parse_bitorder(bytes: &[u8]) -> Ipv4Header {
        let mut reader = BitReader::<Msb0>::new(bytes);
        Ipv4Header {
            version: reader.read_bits(4),
            ihl: reader.read_bits(4),
            dscp: reader.read_bits(6),
            ecn: reader.read_bits(2),
            length: reader.read_bits(16),
            identification: reader.read_bits(16),
            flags: reader.read_bits(3),
            offset: reader.read_bits(13),
            ttl: reader.read_bits(8),
            protocol: reader.read_bits(8),
            checksum: reader.read_bits(16),
        }
    }
}

fn main() {
    const N: usize = 10_000;
    let test_data = vec![
        0x45, 0x00, 0x00, 0x4b, 0x0f, 0x49, 0x00, 0x00, 0x80, 0x11, 0x63, 0xa5,
    ];

    let start = Instant::now();
    for _ in 0..N {
        let ip_header = Ipv4Header::try_from(test_data.as_ref()).unwrap();
    }
    let stop = Instant::now();
    let a = dbg!(stop.duration_since(start));

    let start = Instant::now();
    for _ in 0..N {
        let ip_header = Ipv4Header::parse_bitorder(&test_data);
    }
    let stop = Instant::now();
    let b = dbg!(stop.duration_since(start));

    dbg!(a.as_nanos() / b.as_nanos());
}

#[bench]
fn bench_deku(b: &mut Bencher) {
    let test_data = vec![
        0x45, 0x00, 0x00, 0x4b, 0x0f, 0x49, 0x00, 0x00, 0x80, 0x11, 0x63, 0xa5,
    ];
    b.iter(|| Ipv4Header::try_from(test_data.as_ref()).unwrap());
}

#[bench]
fn bench_bitorder(b: &mut Bencher) {
    let test_data = vec![
        0x45, 0x00, 0x00, 0x4b, 0x0f, 0x49, 0x00, 0x00, 0x80, 0x11, 0x63, 0xa5,
    ];
    b.iter(|| Ipv4Header::parse_bitorder(test_data.as_ref()));
}

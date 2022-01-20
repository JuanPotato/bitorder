# bitorder

```rust
let data = vec![\xCD, \x0A];
let mut b = BitStreamMsb::new(b"\xCD\x0A");

assert_eq!(b.read_bits::<u8>(1), Ok(1u8));
assert_eq!(b.read_bits::<u16>(2), Ok(2u16));
assert_eq!(b.read_bits::<u32>(3), Ok(3u32));
assert_eq!(b.read_bits::<u64>(4), Ok(4u64));
assert_eq!(b.read_bits::<u128>(5), Ok(5u128));
```

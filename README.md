# lz-str-rs
[![crates.io](https://img.shields.io/crates/v/lz-str.svg)](https://crates.io/crates/lz-str)
[![Documentation](https://docs.rs/lz-str/badge.svg)](https://docs.rs/lz-str)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/lz-str.svg)](./LICENSE-APACHE)
![Rust](https://github.com/adumbidiot/lz-str-rs/workflows/Rust/badge.svg)

A port of [lz-string](https://github.com/pieroxy/lz-string) to Rust. 

### Installing

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
lz-str = "0.1.0"
```

## Getting Started

```rust
use lz_str::{
    compress,
    decompress,
};

const DATA_STR: &'static str = "The quick brown fox jumps over the lazy dog";

fn main(){
    let compressed = compress(&DATA_STR);
    let decompressed = decompress(&compressed).expect("Valid Decompress");
    assert_eq!(DATA_STR, String::from_utf16(&decompressed).expect("Valid Unicode String"));
}
```


## Testing
```bash
cargo test
```

## Benching
```bash
cargo bench
```

## Authors
adumbidiot (Nathaniel Daniel)

## License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
# Generic Numbers

Traits for generic number types in Rust.

## Comparison to `num_traits`

This crate is very similar to `num_traits` in its purpose and use cases.
However, `gnum` aims to provide more out of the box, and be a closer match to Rust's
core APIs.

- Instead of methods like `PI()`, `gnum` uses associated constants like `PI`, the same as Rust itself.
- `gnum` aims to maintain a one-to-one mapping with all numeric APIs for Rust's types like `f32`.
- `gnum` not only supports generic math using scalar types like `f32`, it also provides traits for SIMD
  types like `core::simd::Simd` (optional).

In a way, `gnum` is a combination of `num_traits` and `simba` under a more modern and unified interface.

## License

`gnum` is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

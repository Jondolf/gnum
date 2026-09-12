# `gimd`

Generic [SIMD] for stable and nightly Rust.

[SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data

## Description

`gimd` provides `Simd<T, N>` and `Mask<T, N>` types for [SIMD] (_Single Instruction, Multiple Data_)
that works on both stable and nightly Rust, with a unified API that is similar to [`core::simd`].

On stable toolchains, with the `wide` feature enabled, the types internally use the [`wide`] crate,
while on nightly Rust, with the `portable_simd` feature enabled, [`core::simd`] is used.
This allows libraries to write SIMD code once, and have it work on both stable and nightly toolchains,
with [`core::simd`] capabilities being fully opt in.

For math, comparisons, SIMD lane manipulation, and more, the [`gnum`] crate is used.
This allows the SIMD types to be fully generic over the backend, and also provides
built-in cross-platform determinism guarantees, and identical results across SIMD backends.
See the [`gnum`] crate for more information.

[`core::simd`]: https://doc.rust-lang.org/core/simd/index.html
[`wide`]: https://github.com/Lokathor/wide
[`gnum`]: https://crates.io/crates/gnum

## Features

- `Simd<T, N>` and `Mask<T, N>` types similar to [`core::simd`]
- Support for both stable and nightly toolchains
    - The `wide` feature works on both stable and nightly Rust, using [`wide`]
    - The `portable_simd` feature only works on nightly Rust, using [`core::simd`]
- [`gnum`] support for generic numerics and SIMD math
- Cross-platform determinism (via [`gnum`])

## Getting Started

First, add `gimd` to your `Cargo.toml`. By default, the `wide` feature is used,
but on nightly Rust, you may also enable `portable_simd` to gain full [`core::simd`] capabilities.

```toml
[dependencies]
# Replace '*' with the latest version
gimd = "*"

# Nightly alternative:
# gimd = { version = "*", default-features = false, features = ["portable_simd"] }
```

When both `wide` and `portable_simd` are enabled, `portable_simd` takes precedence.

Now, you can start using `gimd`!

```rust
use gimd::{Simd, f32x4};

// f32x4 is an alias of Simd<f32, 4>
let a = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
let b = Simd::<f32, 4>::splat(2.0);

assert_eq!((a * b).to_array(), [2.0, 4.0, 6.0, 8.0]);
```

To support more mathematical operations (`sqrt`, `sin`, `cos`, and so on),
comparison methods, SIMD lane manipulation, and more, you will also need the `gnum` crate:

```rust
use gimd::f32x4;
use gnum::prelude::{NumOrd, Real, Zero};

let angles = f32x4::from_array([-1.0, 0.0, 1.0, 2.0]);
let (sines, cosines) = angles.sin_cos();
let zero = f32x4::ZERO;

assert_eq!(sines.num_gt(zero).to_array(), [false, false, true, true]);
assert_eq!(cosines.num_gt(zero).to_array(), [true, true, true, false]);
```

See the [`gnum`] crate for more information on the supported operations
and their usage.

## Why `gimd`?

gingle instruction, multiple data

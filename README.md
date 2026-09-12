# `gnum`

Generic numerics with SIMD and determinism in Rust.

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [SIMD](#simd)
- [Determinism](#determinism)
- [`no_std` Support](#no_std-support)
- [Related Crates](#related-crates)
- [License](#license)

## Overview

`gnum` is a generic numerics crate for Rust, with support for SIMD (both [`wide`] and [`core::simd`]),
cross-platform determinism, and portability.

The following core traits are provided for numeric types:

- `Num` for all numeric types
- `Int` for signed and unsigned integers
- `Real` for real numbers, either floating-point or fixed-point (only floating-point is currently implemented)
- `Float` for floating-point real numbers
- `Signed` for signed numbers

`gnum` is designed to be generic across both scalar and SIMD types,
and provides traits like the following for lane-wise abstractions:

- `SimdLike` for all SIMD-like and scalar number types
- `MaskLike` for a lane-wise generalization of `bool`
- `NumEq` for a lane-wise generalization of `PartialEq`/`Eq`
- `NumOrd` for a lane-wise generalization of `PartialOrd`/`Ord`
- `Select` for choosing elements from two SIMD-like values using a mask
- `NumCast` for casting between numeric types with the semantics of the `as` operator

More SIMD-specific traits include:

- `Reduce` for collapsing the lanes of a SIMD value into a single scalar value
- `ReduceBitwise` for bitwise reductions for integer SIMD types
- `Shuffle4` and `Shuffle8` for lane-wise shuffling of SIMD values
- `Swizzle` for common lane permutations like reversing, rotating, shifting, and interleaving
- `Transpose` and `TransposeRows` for transposing square matrices stored as SIMD row arrays
- `MaskCast` for casting between mask types and their integer representations

There are also traits for checked, saturating, and wrapping operations,
among other useful methods for generic numerics in Rust.

What sets `gnum` apart from crates like [`num-traits`] is being able to generalize
over scalar and SIMD types, with built-in support for bit-identical and cross-platform
deterministic results across both of them. See [SIMD](#simd) and [Determinism](#determinism)
for more information.

[`core::simd`]: https://doc.rust-lang.org/core/simd/index.html
[`wide`]: https://crates.io/crates/wide
[`num-traits`]: https://crates.io/crates/num-traits

## Getting Started

Add `gnum` to your `Cargo.toml`:

```toml
[dependencies]
# Replace `*` with the latest version
gnum = "*"
```

You can now start using the traits provided by `gnum` to write generic numeric code.
For example, consider a 2D vector type with a generic number type and some basic operations:

```rust
use gnum::num::Num;

#[derive(Clone, Copy)]
struct Vec2<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> Vec2<T> {
    const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn length_squared(self) -> T {
        self.dot(self)
    }
}
```

The `Vec2<T>` type can then be used with `f32`, `i32`, `u16`, or any other `Num` type.
With `T: Real`, we also gain access to math that is only available on real numbers:

```rust
use gnum::num::Real;

impl<T: Real> Vec2<T> {
    fn length(self) -> T {
        self.length_squared().sqrt()
    }

    fn normalize(self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
}
```

The traits provided by `gnum` cover almost all numerical operations in the standard library,
which makes it straightforward to express almost any mathematical code generically.

## SIMD

`gnum` supports [SIMD] (_Single Instruction, Multiple Data_) for both [`core::simd`] (nightly)
and the [`wide`] crate (stable). Almost all traits in `gnum` are implemented for both scalar
and SIMD types, allowing code to be generic across both.

For example, consider the earlier `Vec2<T>` type. We can use both the scalar `f32`
and SIMD `f32x4` type with it, and notably, the results are guaranteed to be bit-identical:

```rust
#![feature(portable_simd)]

use core::simd::f32x4;
use gnum::num::Real;

// Create 4 vectors
let v1: Vec2<f32> = Vec2::new(1.0, 2.0);
let v2: Vec2<f32> = Vec2::new(3.0, 4.0);
let v3: Vec2<f32> = Vec2::new(5.0, 6.0);
let v4: Vec2<f32> = Vec2::new(7.0, 8.0);

// Create an equivalent "wide" SIMD vector
let v_wide: Vec2<f32x4> = Vec2::new(
    f32x4::from_array([1.0, 3.0, 5.0, 7.0]),
    f32x4::from_array([2.0, 4.0, 6.0, 8.0]),
);

// Compute the lengths
let lengths = [v1.length(), v2.length(), v3.length(), v4.length()];
let length_wide = v_wide.length();

// Results are bit-identical
assert_eq!(lengths, length_wide.to_array());
```

See [Determinism](#determinism) for more information about the determinism
contracts provided by `gnum`.

[SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data

## Determinism

On IEEE-754 compliant hardware, all operations in `gnum` are guaranteed to be
deterministic and bit-identical across both scalar and SIMD implementations
for the same element type, except where otherwise noted. For example,
calling `Real::sqrt` on four scalar `f32` values is guaranteed to produce
the same results as calling `Real::sqrt` on an `f32x4` SIMD value.

Some operations such as transcendental functions (`sin`, `cos`, `atan2`)
are non-deterministic in both the standard library and in `gnum`, and are
documented as such. For these methods, `gnum` provides "stable" alternatives
such as `Real::sin_stable`, which are deterministic and guaranteed to produce
the same results across both scalar and SIMD implementations for the same element type.
Some of these stable alternatives may have lower precision or be slightly slower than
the standard versions for scalar types, but are often competitive or even _faster_ for SIMD types.

Note that these determinism guarantees only apply to a given version of `gnum`
and the Rust compiler. Do not expect bit-identical results across different versions.

If these determinism guarantees are not met for some operation, it is considered
a bug and should be reported on the [issue tracker].

[issue tracker]: https://github.com/Jondolf/gnum/issues

### NaN Bit Patterns

As with the standard library, `gnum` does not generally guarantee that NaN values
returned from arithmetic operations have any particular sign, quiet bit, or payload,
or that the bit pattern matches between scalar and SIMD implementations. This applies
to both the standard and "stable" versions of operations.

Do not rely on the exact bit patterns of NaN values returned from arithmetic operations.
Treat them as unspecified and non-deterministic.

See the [`f32` documentation on NaN bit patterns][NaN bit patterns] for more information.

[NaN bit patterns]: https://doc.rust-lang.org/std/primitive.f32.html#nan-bit-patterns

## `no_std` Support

`gnum` supports `no_std` when the default `std` feature is disabled.
The determinism guarantees described in [Determinism](#determinism) are still upheld.

Some operations like `sin` are normally only available in `std` environments.
With `no_std`, these operations internally use the portable `_stable` alternatives
described in [Determinism](#determinism). These can produce different results
than the `std` versions, and may have lower precision or be slightly slower.

If you need `no_std` math that may be more accurate but does not work with SIMD types,
consider checking out [`libm`].

[`libm`]: https://crates.io/crates/libm

## Related Crates

- [`gela`] is a generic linear algebra library using `gnum`, and provides vectors, matrices,
  rotations, isometries, affine transformations, and more.
- [`gimd`] is a generic SIMD library using `gnum`, and provides `Simd<T, N>` and `Mask<T, N>`
  types that work on both stable and nightly Rust.

[`gela`]: https://crates.io/crates/gela
[`gimd`]: https://crates.io/crates/gimd

## License

`gnum` is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

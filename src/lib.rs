//! Generic numerics with [SIMD](#simd) and [determinism](#determinism) in Rust.
//!
//! # Getting Started
//!
//! Add `gnum` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # Replace `*` with the latest version
//! gnum = "*"
//! ```
//!
//! You can now start using the traits provided by `gnum` to write generic numeric code.
//! For example, consider a 2D vector type with a generic number type and some basic operations:
//!
//! ```rust
//! use gnum::num::Num;
//!
//! #[derive(Clone, Copy)]
//! struct Vec2<T: Num> {
//!     x: T,
//!     y: T,
//! }
//!
//! impl<T: Num> Vec2<T> {
//!     const fn new(x: T, y: T) -> Self {
//!         Self { x, y }
//!     }
//!
//!     fn dot(self, other: Self) -> T {
//!         self.x * other.x + self.y * other.y
//!     }
//!
//!     fn length_squared(self) -> T {
//!         self.dot(self)
//!     }
//! }
//! ```
//!
//! The `Vec2<T>` type can then be used with `f32`, `i32`, `u16`, or any other [`Num`] types.
//! With <code>T: [Real]</code>, we also gain access to math that is only available on real numbers:
//!
//! ```rust
//! use gnum::num::Real;
//! # use gnum::num::Num;
//! #
//! # #[derive(Clone, Copy)]
//! # struct Vec2<T: Num> {
//! #     x: T,
//! #     y: T,
//! # }
//! # impl<T: Num> Vec2<T> {
//! #     fn length_squared(self) -> T {
//! #         self.x * self.x + self.y * self.y
//! #     }
//! # }
//!
//! impl<T: Real> Vec2<T> {
//!     fn length(self) -> T {
//!         self.length_squared().sqrt()
//!     }
//!
//!     fn normalize(self) -> Self {
//!         let len = self.length();
//!         Self {
//!             x: self.x / len,
//!             y: self.y / len,
//!         }
//!     }
//! }
//! ```
//!
//! The traits provided by `gnum` cover almost all numerical operations in the standard library,
//! which makes it straightforward to express almost any mathematical code generically.
//!
//! # Traits
//!
//! `gnum` provides the following core traits for numeric types:
//!
//! - [`Num`] for all numeric types
//! - [`Int`] for signed and unsigned integers
//! - [`Real`] for real numbers, either floating-point or fixed-point (only floating-point is currently implemented)
//! - [`Float`] for floating-point real numbers
//! - [`Signed`] for signed numbers
//!
//! `gnum` is designed to be generic across both scalar and SIMD types,
//! and provides traits like the following for lane-wise abstractions:
//!
//! - [`SimdLike`] for all SIMD-like and scalar number types
//! - [`MaskLike`] for a lane-wise generalization of `bool`
//! - [`NumEq`] for a lane-wise generalization of [`PartialEq`]/[`Eq`]
//! - [`NumOrd`] for a lane-wise generalization of [`PartialOrd`]/[`Ord`]
//! - [`Select`] for choosing elements from two SIMD-like values using a mask
//! - [`NumCast`] for casting between numeric types with the semantics of the `as` operator
//!
//! More SIMD-specific traits include:
//!
//! - [`Reduce`] for collapsing the lanes of a SIMD value into a single scalar value
//! - [`ReduceBitwise`] for bitwise reductions for integer SIMD types
//! - [`Shuffle4`] and [`Shuffle8`] for lane-wise shuffling of SIMD values
//! - [`Swizzle`] for common lane permutations like reversing, rotating, shifting, and interleaving
//! - [`Transpose`] and [`TransposeRows`] for transposing square matrices stored as SIMD row arrays
//! - [`MaskCast`] for casting between mask types and their integer representations
//!
//! To scope generic code to only scalar or only SIMD types, the following traits are provided:
//!
//! - [`ScalarValue`]/[`SimdValue`]
//! - [`ScalarNum`]/[`SimdNum`]
//! - [`ScalarReal`]/[`SimdReal`]
//! - [`ScalarFloat`]/[`SimdFloat`]
//! - [`ScalarInt`]/[`SimdInt`]
//!
//! Additional traits that may not be supported by all types include:
//!
//! - Euclidean division and remainder: [`DivEuclid`], [`RemEuclid`]
//! - Checked ops: [`CheckedAdd`], [`CheckedSub`], [`CheckedMul`], [`CheckedDiv`],
//!   [`CheckedRem`], [`CheckedDivEuclid`], [`CheckedRemEuclid`], [`CheckedNeg`],
//!   [`CheckedShl`], [`CheckedShr`], [`CheckedAbs`], [`CheckedPow`]
//! - Wrapping ops: [`WrappingAdd`], [`WrappingSub`], [`WrappingMul`], [`WrappingDiv`],
//!   [`WrappingRem`], [`WrappingDivEuclid`], [`WrappingRemEuclid`], [`WrappingNeg`],
//!   [`WrappingShl`], [`WrappingShr`], [`WrappingAbs`], [`WrappingPow`]
//! - Saturating ops: [`SaturatingAdd`], [`SaturatingSub`], [`SaturatingMul`], [`SaturatingDiv`],
//!   [`SaturatingNeg`], [`SaturatingAbs`], [`SaturatingPow`]
//! - Byte conversions: [`ToBytes`]
//!
//! # SIMD
//!
//! `gnum` supports [SIMD] (_Single Instruction, Multiple Data_) for both [`core::simd`] (nightly)
//! and the [`wide`] crate (stable). Almost all traits in `gnum` are implemented for both scalar
//! and SIMD types, allowing code to be generic across both.
//!
//! For example, consider the earlier `Vec2<T>` type. We can use both the scalar `f32`
//! and SIMD `f32x4` type with it, and notably, the results are guaranteed to be bit-identical:
//!
//! ```rust
//! #![feature(portable_simd)]
//!
//! use core::simd::f32x4;
//! use gnum::num::Real;
//! #
//! # #[derive(Clone, Copy)]
//! # struct Vec2<T: Real> {
//! #     x: T,
//! #     y: T,
//! # }
//! #
//! # impl<T: Real> Vec2<T> {
//! #     const fn new(x: T, y: T) -> Self {
//! #         Self { x, y }
//! #     }
//! #
//! #     fn length(self) -> T {
//! #         (self.x * self.x + self.y * self.y).sqrt()
//! #     }
//! # }
//!
//! // Create 4 vectors
//! let v1: Vec2<f32> = Vec2::new(1.0, 2.0);
//! let v2: Vec2<f32> = Vec2::new(3.0, 4.0);
//! let v3: Vec2<f32> = Vec2::new(5.0, 6.0);
//! let v4: Vec2<f32> = Vec2::new(7.0, 8.0);
//!
//! // Create an equivalent "wide" SIMD vector
//! let v_wide: Vec2<f32x4> = Vec2::new(
//!     f32x4::from_array([1.0, 3.0, 5.0, 7.0]),
//!     f32x4::from_array([2.0, 4.0, 6.0, 8.0]),
//! );
//!
//! // Compute the lengths
//! let lengths = [v1.length(), v2.length(), v3.length(), v4.length()];
//! let length_wide = v_wide.length();
//!
//! // Results are bit-identical
//! assert_eq!(lengths, length_wide.to_array());
//! ```
//!
//! See [Determinism](#determinism) for more information about the determinism
//! contracts provided by `gnum`.
//!
//! [SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data
//! [`wide`]: https://crates.io/crates/wide
//!
//! # Determinism
//!
//! On IEEE-754 compliant hardware, all operations in `gnum` are guaranteed to be
//! deterministic and bit-identical across both scalar and SIMD implementations
//! for the same element type, except where otherwise noted. For example,
//! calling [`Real::sqrt`] on four scalar `f32` values is guaranteed to produce
//! the same results as calling [`Real::sqrt`] on an `f32x4` SIMD value.
//!
//! Some operations such as transcendental functions (`sin`, `cos`, `atan2`)
//! are non-deterministic in both the standard library and in `gnum`, and are
//! documented as such. For these methods, `gnum` provides "stable" alternatives
//! such as [`Real::sin_stable`], which are deterministic and guaranteed to produce
//! the same results across both scalar and SIMD implementations for the same element type.
//! Some of these stable alternatives may have lower precision or be slightly slower than
//! the standard versions for scalar types, but are often competitive or even _faster_ for SIMD types.
//!
//! Note that these determinism guarantees only apply to a given version of `gnum`
//! and the Rust compiler. Do not expect bit-identical results across different versions.
//!
//! If these determinism guarantees are not met for some operation, it is considered
//! a bug and should be reported on the [issue tracker].
//!
//! [issue tracker]: https://github.com/Jondolf/gnum/issues
//!
//! ## NaN Bit Patterns
//!
//! As with the standard library, `gnum` does not generally guarantee that NaN values
//! returned from arithmetic operations have any particular sign, quiet bit, or payload,
//! or that the bit pattern matches between scalar and SIMD implementations. This applies
//! to both the standard and "stable" versions of operations.
//!
//! Do not rely on the exact bit patterns of NaN values returned from arithmetic operations.
//! Treat them as unspecified and non-deterministic.
//!
//! See the [`f32` documentation on NaN bit patterns][NaN bit patterns] for more information.
//!
//! [NaN bit patterns]: https://doc.rust-lang.org/std/primitive.f32.html#nan-bit-patterns
//!
//! # `no_std` Support
//!
//! `gnum` supports `no_std` when the default `std` feature is disabled.
//! The determinism guarantees described in [Determinism](#determinism) are still upheld.
//!
//! Some operations like `sin` are normally only available in `std` environments.
//! With `no_std`, these operations internally use the portable `_stable` alternatives
//! described in [Determinism](#determinism). These can produce different results
//! than the `std` versions, and may have lower precision or be slightly slower.
//!
//! If you need `no_std` math that may be more accurate but does not work with SIMD types,
//! consider checking out [`libm`].
//!
//! [`libm`]: https://crates.io/crates/libm

#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
#![cfg_attr(
    all(feature = "portable_simd", not(feature = "std")),
    feature(core_intrinsics)
)]
#![cfg_attr(
    all(feature = "portable_simd", not(feature = "std")),
    expect(internal_features)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::wrong_self_convention)]
#![warn(missing_docs)]

#[expect(unused_imports, reason = "for doc links")]
#[doc(hidden)]
use prelude::*;

mod bytes;
mod impl_traits;

pub mod cmp;
pub mod num;
pub mod simd;

pub use bytes::ToBytes;

/// A collection of commonly used `gnum` types and traits, re-exported for convenience.
pub mod prelude {
    pub use crate::cmp::{NumEq, NumOrd};
    pub use crate::num::{
        Float, Int, NegOne, Num, NumCast, One, Real, RealConstants, ScalarFloat, ScalarInt,
        ScalarNum, ScalarReal, ScalarValue, Signed, SimdFloat, SimdInt, SimdNum, SimdReal,
        SimdValue, Zero, ops::*,
    };
    pub use crate::simd::{
        MaskCast, MaskLike, Reduce, ReduceBitwise, Select, Shuffle4, Shuffle8, SimdLike, Swizzle,
        Transpose, TransposeRows,
    };
}

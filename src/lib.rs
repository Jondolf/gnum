//! Generic numerics for Rust.
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
//! For example, consider a 2D vector type with generic [`Real`] components, and a function
//! to compute the distance between two vectors:
//!
//! ```rust
//! use gnum::num::{Float, Real};
//!
//! #[derive(Clone, Copy)]
//! struct Vec2<T: Real> {
//!     x: T,
//!     y: T,
//! }
//!
//! impl<T: Real> Vec2<T> {
//!     fn distance(self, other: Self) -> T {
//!         let dx = self.x - other.x;
//!         let dy = self.y - other.y;
//!         (dx * dx + dy * dy).sqrt()
//!     }
//! }
//! ```
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
//! - [`Transpose`] for transposing square matrices stored as SIMD row arrays
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
//! `gnum` supports [SIMD] for both [`core::simd`] (nightly) and the [`wide`] crate (stable).
//! Almost all traits in `gnum` are implemented for both scalar and SIMD types,
//! allowing code to be generic across both.
//!
//! [SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data
//!
//! # Determinism
//!
//! On IEEE-754 compliant hardware, all operations in `gnum` are guaranteed to be
//! deterministic and bit-identical across both scalar and SIMD implementations
//! for the same element type, except where otherwise noted. For example,
//! calling [`Real::sqrt`] on four scalar `f32` values is guaranteed to produce
//! the same results as calling [`Real::sqrt`] on a `f32x4` SIMD value.
//!
//! Some operations such as transcendental functions (`sin`, `cos`, `atan2`)
//! are non-deterministic in both the standard library and in `gnum`, and are
//! documented as such. For these methods, `gnum` provides "stable" alternatives
//! such as [`Real::sin_stable`], which are deterministic and guaranteed to produce
//! the same results across both scalar and SIMD implementations for the same element type.
//! These stable alternatives may be slightly slower than the standard versions for scalar types,
//! but are often competitive or even _faster_ for SIMD types.
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

#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
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
        Float, Int, Num, NumCast, Real, RealConstants, ScalarFloat, ScalarInt, ScalarNum,
        ScalarReal, ScalarValue, Signed, SimdFloat, SimdInt, SimdNum, SimdReal, SimdValue, ops::*,
    };
    pub use crate::simd::{
        MaskCast, MaskLike, Reduce, ReduceBitwise, Select, Shuffle4, Shuffle8, SimdLike, Swizzle,
        Transpose,
    };
}

//! Traits for abstracting [SIMD] operations over both scalar and SIMD types.
//!
//! [SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data

mod mask_cast;
mod mask_like;
mod reduce;
mod select;
mod shuffle;
mod simd_like;
mod swizzle;
mod transpose;

pub use mask_cast::MaskCast;
pub use mask_like::MaskLike;
#[cfg(any(feature = "portable_simd", feature = "wide"))]
pub(crate) use reduce::generic_reduce_stable;
pub use reduce::{Reduce, ReduceBitwise};
pub use select::Select;
pub use shuffle::{Shuffle4, Shuffle8};
pub use simd_like::SimdLike;
pub use swizzle::Swizzle;
pub use transpose::Transpose;
#[cfg(any(feature = "portable_simd", feature = "wide"))]
pub(crate) use transpose::{blocked_transpose, generic_transpose};

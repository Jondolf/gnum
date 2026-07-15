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

pub use mask_cast::MaskCast;
pub use mask_like::MaskLike;
pub use reduce::{Reduce, ReduceBitwise};
pub use select::Select;
pub use shuffle::{Shuffle4, Shuffle8};
pub use simd_like::SimdLike;
pub use swizzle::Swizzle;

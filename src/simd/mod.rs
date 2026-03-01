//! Traits for abstracting [SIMD] operations over both scalar and SIMD types.
//!
//! [SIMD]: https://en.wikipedia.org/wiki/Single_instruction,_multiple_data

mod mask_like;
mod select;
mod simd_like;

pub use mask_like::MaskLike;
pub use select::Select;
pub use simd_like::SimdLike;

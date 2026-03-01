//! Generic numerics for Rust.

// Experimental APIs
#![cfg_attr(feature = "more_float_constants", feature(more_float_constants))]
#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
#![allow(clippy::wrong_self_convention)]

mod bytes;
mod impl_traits;

pub mod cmp;
pub mod num;
pub mod simd;

pub use bytes::ToBytes;

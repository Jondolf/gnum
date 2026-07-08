//! Generic numerics for Rust.

#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
#![allow(clippy::wrong_self_convention)]
#![warn(missing_docs)]

mod bytes;
mod impl_traits;

pub mod cmp;
pub mod num;
pub mod simd;

pub use bytes::ToBytes;

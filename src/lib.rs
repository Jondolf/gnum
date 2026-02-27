//! Generic numerics for Rust.

// Experimental APIs
#![cfg_attr(feature = "more_float_constants", feature(more_float_constants))]
#![cfg_attr(feature = "float_algebraic", feature(float_algebraic))]
#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
#![allow(clippy::wrong_self_convention)]

pub mod simd;
pub mod traits;

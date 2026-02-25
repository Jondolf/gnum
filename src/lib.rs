//! Generic numerics for Rust.

// Experimental APIs
#![cfg_attr(feature = "more_float_constants", feature(more_float_constants))]
#![cfg_attr(feature = "float_gamma", feature(float_gamma))]
#![cfg_attr(feature = "float_erf", feature(float_erf))]
#![cfg_attr(feature = "float_minimum_maximum", feature(float_minimum_maximum))]
#![cfg_attr(feature = "clamp_magnitude", feature(clamp_magnitude))]
#![cfg_attr(feature = "float_algebraic", feature(float_algebraic))]
#![cfg_attr(feature = "portable_simd", feature(portable_simd))]

pub mod bytes;
pub mod cmp;
pub mod float;
pub mod num;
pub mod simd;

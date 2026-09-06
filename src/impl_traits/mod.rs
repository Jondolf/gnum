//! Implementations of `gnum` traits for various types and platforms.

#[cfg(feature = "portable_simd")]
mod portable_simd;

#[cfg(feature = "wide")]
mod wide;

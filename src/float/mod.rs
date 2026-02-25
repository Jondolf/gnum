#[cfg(feature = "float_algebraic")]
mod algebraic;
mod classify;
mod constants;
mod total_cmp;

#[cfg(feature = "float_algebraic")]
pub use algebraic::AlgebraicOps;
pub use classify::FloatClassify;
pub use constants::FloatConst;
pub use total_cmp::TotalCmp;

use crate::{cmp::SimdPartialOrd, num::Num, simd::SimdValue};
use core::ops::Neg;

#[cfg(feature = "portable_simd")]
mod portable_simd;
mod scalar;

// TODO: Implement for core::simd, wide, and fixed
// TODO: cross_platform_determinism and libm features
pub trait Float: Num + Copy + SimdValue + SimdPartialOrd + FloatConst + Neg<Output = Self> {
    type Sign;
    type Int;
    type Bits;

    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: Self;
    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn round_ties_even(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn powi(self, n: Self::Int) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    #[cfg(feature = "float_gamma")]
    fn gamma(self) -> Self;
    #[cfg(feature = "float_gamma")]
    fn ln_gamma(self) -> (Self, Self::Sign);
    #[cfg(feature = "float_erf")]
    fn erf(self) -> Self;
    #[cfg(feature = "float_erf")]
    fn erfc(self) -> Self;
    fn is_nan(self) -> Self::Bool;
    fn is_infinite(self) -> Self::Bool;
    fn is_finite(self) -> Self::Bool;
    fn is_subnormal(self) -> Self::Bool;
    fn is_normal(self) -> Self::Bool;
    fn is_sign_positive(self) -> Self::Bool;
    fn is_sign_negative(self) -> Self::Bool;
    fn next_up(self) -> Self;
    fn next_down(self) -> Self;
    fn recip(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    #[cfg(feature = "float_minimum_maximum")]
    fn maximum(self, other: Self) -> Self;
    #[cfg(feature = "float_minimum_maximum")]
    fn minimum(self, other: Self) -> Self;
    fn midpoint(self, other: Self) -> Self;
    unsafe fn to_int_unchecked(self) -> Self::Int;
    fn to_bits(self) -> Self::Bits;
    fn from_bits(bits: Self::Bits) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    #[cfg(feature = "clamp_magnitude")]
    fn clamp_magnitude(self, limit: Self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn copysign(self, sign: Self) -> Self;
}

use core::simd::{LaneCount, Simd, SupportedLaneCount, cmp::SimdPartialOrd, num::SimdFloat};
use std::simd::StdFloat;

use crate::{
    float::{Float, FloatConst},
    num::Zero,
};

impl<const N: usize> FloatConst for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const E: Self = Self::splat(f32::E);
    const FRAC_1_PI: Self = Self::splat(f32::FRAC_1_PI);
    const FRAC_1_SQRT_2: Self = Self::splat(f32::FRAC_1_SQRT_2);
    const FRAC_2_PI: Self = Self::splat(f32::FRAC_2_PI);
    const FRAC_2_SQRT_PI: Self = Self::splat(f32::FRAC_2_SQRT_PI);
    const FRAC_PI_2: Self = Self::splat(f32::FRAC_PI_2);
    const FRAC_PI_3: Self = Self::splat(f32::FRAC_PI_3);
    const FRAC_PI_4: Self = Self::splat(f32::FRAC_PI_4);
    const FRAC_PI_6: Self = Self::splat(f32::FRAC_PI_6);
    const FRAC_PI_8: Self = Self::splat(f32::FRAC_PI_8);
    const LN_2: Self = Self::splat(f32::LN_2);
    const LN_10: Self = Self::splat(f32::LN_10);
    const LOG2_10: Self = Self::splat(f32::LOG2_10);
    const LOG2_E: Self = Self::splat(f32::LOG2_E);
    const LOG10_2: Self = Self::splat(f32::LOG10_2);
    const LOG10_E: Self = Self::splat(f32::LOG10_E);
    const PI: Self = Self::splat(f32::PI);
    const SQRT_2: Self = Self::splat(f32::SQRT_2);
    const TAU: Self = Self::splat(f32::TAU);
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self = Self::splat(f32::EGAMMA);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self = Self::splat(f32::FRAC_1_SQRT_3);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self = Self::splat(f32::FRAC_1_SQRT_2PI);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self = Self::splat(f32::FRAC_1_SQRT_PI);
    #[cfg(feature = "more_float_constants")]
    const PHI: Self = Self::splat(f32::PHI);
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self = Self::splat(f32::SQRT_3);
}

impl<const N: usize> FloatConst for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const E: Self = Self::splat(f64::E);
    const FRAC_1_PI: Self = Self::splat(f64::FRAC_1_PI);
    const FRAC_1_SQRT_2: Self = Self::splat(f64::FRAC_1_SQRT_2);
    const FRAC_2_PI: Self = Self::splat(f64::FRAC_2_PI);
    const FRAC_2_SQRT_PI: Self = Self::splat(f64::FRAC_2_SQRT_PI);
    const FRAC_PI_2: Self = Self::splat(f64::FRAC_PI_2);
    const FRAC_PI_3: Self = Self::splat(f64::FRAC_PI_3);
    const FRAC_PI_4: Self = Self::splat(f64::FRAC_PI_4);
    const FRAC_PI_6: Self = Self::splat(f64::FRAC_PI_6);
    const FRAC_PI_8: Self = Self::splat(f64::FRAC_PI_8);
    const LN_2: Self = Self::splat(f64::LN_2);
    const LN_10: Self = Self::splat(f64::LN_10);
    const LOG2_10: Self = Self::splat(f64::LOG2_10);
    const LOG2_E: Self = Self::splat(f64::LOG2_E);
    const LOG10_2: Self = Self::splat(f64::LOG10_2);
    const LOG10_E: Self = Self::splat(f64::LOG10_E);
    const PI: Self = Self::splat(f64::PI);
    const SQRT_2: Self = Self::splat(f64::SQRT_2);
    const TAU: Self = Self::splat(f64::TAU);
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self = Self::splat(f64::EGAMMA);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self = Self::splat(f64::FRAC_1_SQRT_3);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self = Self::splat(f64::FRAC_1_SQRT_2PI);
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self = Self::splat(f64::FRAC_1_SQRT_PI);
    #[cfg(feature = "more_float_constants")]
    const PHI: Self = Self::splat(f64::PHI);
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self = Self::splat(f64::SQRT_3);
}

macro_rules! impl_float_simd {
    ($float:tt, $int:ty) => {
        impl<const N: usize> Float for Simd<$float, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            type Sign = Simd<i32, N>;
            type Int = Simd<i32, N>;
            type Bits = <Self as SimdFloat>::Bits;

            const RADIX: u32 = $float::RADIX;
            const MANTISSA_DIGITS: u32 = $float::MANTISSA_DIGITS;
            const DIGITS: u32 = $float::DIGITS;
            const EPSILON: Self = Self::splat($float::EPSILON);
            const MIN: Self = Self::splat($float::MIN);
            const MIN_POSITIVE: Self = Self::splat($float::MIN_POSITIVE);
            const MAX: Self = Self::splat($float::MAX);
            const MIN_EXP: i32 = $float::MIN_EXP;
            const MAX_EXP: i32 = $float::MAX_EXP;
            const MIN_10_EXP: i32 = $float::MIN_10_EXP;
            const MAX_10_EXP: i32 = $float::MAX_10_EXP;
            const NAN: Self = Self::splat($float::NAN);
            const INFINITY: Self = Self::splat($float::INFINITY);
            const NEG_INFINITY: Self = Self::splat($float::NEG_INFINITY);

            fn floor(self) -> Self {
                StdFloat::floor(self)
            }
            fn ceil(self) -> Self {
                StdFloat::ceil(self)
            }
            fn round(self) -> Self {
                StdFloat::round(self)
            }
            fn round_ties_even(self) -> Self {
                self.as_array().map(|x| x.round_ties_even()).into()
            }
            fn trunc(self) -> Self {
                StdFloat::trunc(self)
            }
            fn fract(self) -> Self {
                StdFloat::fract(self)
            }
            fn mul_add(self, a: Self, b: Self) -> Self {
                StdFloat::mul_add(self, a, b)
            }
            fn div_euclid(self, rhs: Self) -> Self {
                let q = Float::trunc(self / rhs);
                let r = self - q * rhs;

                let mask = r.simd_lt(Self::ZERO);
                let correction = Float::signum(rhs);
                mask.select(q - correction, q)
            }
            fn rem_euclid(self, rhs: Self) -> Self {
                let r = self % rhs;
                let mask = r.simd_lt(Self::ZERO);
                let adjusted = r + Float::abs(rhs);
                mask.select(adjusted, r)
            }
            fn powi(self, n: Self::Int) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].powi(n[i]);
                }
                result
            }
            fn powf(self, n: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].powf(n[i]);
                }
                result
            }
            fn sqrt(self) -> Self {
                StdFloat::sqrt(self)
            }
            fn exp(self) -> Self {
                StdFloat::exp(self)
            }
            fn exp2(self) -> Self {
                StdFloat::exp2(self)
            }
            fn ln(self) -> Self {
                StdFloat::ln(self)
            }
            fn log(self, base: Self) -> Self {
                StdFloat::log(self, base)
            }
            fn log2(self) -> Self {
                StdFloat::log2(self)
            }
            fn log10(self) -> Self {
                StdFloat::log10(self)
            }
            fn cbrt(self) -> Self {
                self.as_array().map(|x| x.cbrt()).into()
            }
            fn hypot(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].hypot(other[i]);
                }
                result
            }
            fn sin(self) -> Self {
                StdFloat::sin(self)
            }
            fn cos(self) -> Self {
                StdFloat::cos(self)
            }
            fn tan(self) -> Self {
                self.as_array().map(|x| x.tan()).into()
            }
            fn asin(self) -> Self {
                self.as_array().map(|x| x.asin()).into()
            }
            fn acos(self) -> Self {
                self.as_array().map(|x| x.acos()).into()
            }
            fn atan(self) -> Self {
                self.as_array().map(|x| x.atan()).into()
            }
            fn atan2(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].atan2(other[i]);
                }
                result
            }
            fn sin_cos(self) -> (Self, Self) {
                let mut sin = self;
                let mut cos = self;
                for i in 0..Self::LEN {
                    let (s, c) = sin[i].sin_cos();
                    sin[i] = s;
                    cos[i] = c;
                }
                (sin, cos)
            }
            fn exp_m1(self) -> Self {
                self.as_array().map(|x| x.exp_m1()).into()
            }
            fn ln_1p(self) -> Self {
                self.as_array().map(|x| x.ln_1p()).into()
            }
            fn sinh(self) -> Self {
                self.as_array().map(|x| x.sinh()).into()
            }
            fn cosh(self) -> Self {
                self.as_array().map(|x| x.cosh()).into()
            }
            fn tanh(self) -> Self {
                self.as_array().map(|x| x.tanh()).into()
            }
            fn asinh(self) -> Self {
                self.as_array().map(|x| x.asinh()).into()
            }
            fn acosh(self) -> Self {
                self.as_array().map(|x| x.acosh()).into()
            }
            fn atanh(self) -> Self {
                self.as_array().map(|x| x.atanh()).into()
            }
            #[cfg(feature = "float_gamma")]
            fn gamma(self) -> Self {
                self.as_array().map(|x| x.gamma()).into()
            }
            #[cfg(feature = "float_gamma")]
            fn ln_gamma(self) -> (Self, Self::Sign) {
                let mut ln_gamma = self;
                let mut sign = Simd::splat(0);
                for i in 0..Self::LEN {
                    let (lg, s) = ln_gamma[i].ln_gamma();
                    ln_gamma[i] = lg;
                    sign[i] = s;
                }
                (ln_gamma, sign)
            }
            #[cfg(feature = "float_erf")]
            fn erf(self) -> Self {
                self.as_array().map(|x| x.erf()).into()
            }
            #[cfg(feature = "float_erf")]
            fn erfc(self) -> Self {
                self.as_array().map(|x| x.erfc()).into()
            }
            fn is_nan(self) -> Self::Bool {
                SimdFloat::is_nan(self)
            }
            fn is_infinite(self) -> Self::Bool {
                SimdFloat::is_infinite(self)
            }
            fn is_finite(self) -> Self::Bool {
                SimdFloat::is_finite(self)
            }
            fn is_subnormal(self) -> Self::Bool {
                SimdFloat::is_subnormal(self)
            }
            fn is_normal(self) -> Self::Bool {
                SimdFloat::is_normal(self)
            }
            fn is_sign_positive(self) -> Self::Bool {
                SimdFloat::is_sign_positive(self)
            }
            fn is_sign_negative(self) -> Self::Bool {
                SimdFloat::is_sign_negative(self)
            }
            fn next_up(self) -> Self {
                self.as_array().map(|x| x.next_up()).into()
            }
            fn next_down(self) -> Self {
                self.as_array().map(|x| x.next_down()).into()
            }
            fn recip(self) -> Self {
                SimdFloat::recip(self)
            }
            fn to_degrees(self) -> Self {
                SimdFloat::to_degrees(self)
            }
            fn to_radians(self) -> Self {
                SimdFloat::to_radians(self)
            }
            fn max(self, other: Self) -> Self {
                SimdFloat::simd_max(self, other)
            }
            fn min(self, other: Self) -> Self {
                SimdFloat::simd_min(self, other)
            }
            #[cfg(feature = "float_minimum_maximum")]
            fn maximum(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].maximum(other[i]);
                }
                result
            }
            #[cfg(feature = "float_minimum_maximum")]
            fn minimum(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].minimum(other[i]);
                }
                result
            }
            fn midpoint(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].midpoint(other[i]);
                }
                result
            }
            unsafe fn to_int_unchecked(self) -> Self::Int {
                let mut result = Self::Int::splat(0);
                for i in 0..Self::LEN {
                    result[i] = unsafe { self[i].to_int_unchecked() };
                }
                result
            }
            fn to_bits(self) -> Self::Bits {
                SimdFloat::to_bits(self)
            }
            fn from_bits(bits: Self::Bits) -> Self {
                SimdFloat::from_bits(bits)
            }
            fn clamp(self, min: Self, max: Self) -> Self {
                SimdFloat::simd_clamp(self, min, max)
            }
            #[cfg(feature = "clamp_magnitude")]
            fn clamp_magnitude(self, limit: Self) -> Self {
                // TODO: Is this bit-for-bit equivalent?
                let abs = Float::abs(self);
                let mask = abs.simd_gt(limit);
                let clamped = self * (limit / abs);
                mask.select(clamped, self)
            }
            fn abs(self) -> Self {
                SimdFloat::abs(self)
            }
            fn signum(self) -> Self {
                SimdFloat::signum(self)
            }
            fn copysign(self, sign: Self) -> Self {
                SimdFloat::copysign(self, sign)
            }
        }
    };
}

impl_float_simd!(f32, i32);
impl_float_simd!(f64, i64);

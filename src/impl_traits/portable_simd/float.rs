use crate::cmp::NumOrd;
use crate::num::{Float, Real, RealConstants, Signed, round::RoundOps};
use crate::simd::Select;
use core::simd::{
    Simd, SimdElement,
    cmp::{SimdPartialEq, SimdPartialOrd},
    num::{SimdFloat, SimdInt, SimdUint},
};
#[cfg(feature = "std")]
use std::simd::StdFloat;

impl<T: SimdElement + RealConstants, const N: usize> RealConstants for Simd<T, N> {
    const HALF: Self = Self::splat(T::HALF);
    const E: Self = Self::splat(T::E);
    const EULER_GAMMA: Self = Self::splat(T::EULER_GAMMA);
    const FRAC_1_PI: Self = Self::splat(T::FRAC_1_PI);
    const FRAC_1_SQRT_2: Self = Self::splat(T::FRAC_1_SQRT_2);
    const FRAC_2_PI: Self = Self::splat(T::FRAC_2_PI);
    const FRAC_2_SQRT_PI: Self = Self::splat(T::FRAC_2_SQRT_PI);
    const FRAC_PI_2: Self = Self::splat(T::FRAC_PI_2);
    const FRAC_PI_3: Self = Self::splat(T::FRAC_PI_3);
    const FRAC_PI_4: Self = Self::splat(T::FRAC_PI_4);
    const FRAC_PI_6: Self = Self::splat(T::FRAC_PI_6);
    const FRAC_PI_8: Self = Self::splat(T::FRAC_PI_8);
    const GOLDEN_RATIO: Self = Self::splat(T::GOLDEN_RATIO);
    const LN_2: Self = Self::splat(T::LN_2);
    const LN_10: Self = Self::splat(T::LN_10);
    const LOG2_10: Self = Self::splat(T::LOG2_10);
    const LOG2_E: Self = Self::splat(T::LOG2_E);
    const LOG10_2: Self = Self::splat(T::LOG10_2);
    const LOG10_E: Self = Self::splat(T::LOG10_E);
    const PI: Self = Self::splat(T::PI);
    const SQRT_2: Self = Self::splat(T::SQRT_2);
    const TAU: Self = Self::splat(T::TAU);
}

macro_rules! impl_real_simd {
    ($($real:tt),+) => {
        $(
        impl<const N: usize> Real for Simd<$real, N> {
            type I32 = Simd<i32, N>;

            #[inline]
            fn from_f32(n: f32) -> Self {
                Self::splat(n as $real)
            }
            #[inline]
            fn from_f64(n: f64) -> Self {
                Self::splat(n as $real)
            }
            // These route through `RoundOps`, which uses the hardware rounding instruction
            // for targets that have one, and a custom int round-trip emulation elsewhere.
            #[inline]
            fn floor(self) -> Self {
                RoundOps::floor_internal(self)
            }
            #[inline]
            fn ceil(self) -> Self {
                RoundOps::ceil_internal(self)
            }
            #[inline]
            fn round(self) -> Self {
                RoundOps::round_internal(self)
            }
            #[inline]
            fn round_ties_even(self) -> Self {
                RoundOps::round_ties_even_internal(self)
            }
            #[inline]
            fn trunc(self) -> Self {
                RoundOps::trunc_internal(self)
            }
            #[inline]
            fn fract(self) -> Self {
                self - RoundOps::trunc_internal(self)
            }
            #[inline]
            fn sqrt(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::sqrt(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    unsafe { core::intrinsics::simd::simd_fsqrt(self) }
                }
            }
            #[inline]
            fn exp(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::exp(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::exp(self)
                }
            }
            #[inline]
            fn exp_stable(self) -> Self {
                crate::num::stable::exp(self)
            }
            #[inline]
            fn exp2(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::exp2(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::exp2(self)
                }
            }
            #[inline]
            fn exp2_stable(self) -> Self {
                crate::num::stable::exp2(self)
            }
            #[inline]
            fn log(self, base: Self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::log(self, base)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::log(self, base)
                }
            }
            #[inline]
            fn log_stable(self, base: Self) -> Self {
                crate::num::stable::log(self, base)
            }
            #[inline]
            fn ln(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::ln(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::ln(self)
                }
            }
            #[inline]
            fn ln_stable(self) -> Self {
                crate::num::stable::ln(self)
            }
            #[inline]
            fn log2(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::log2(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::log2(self)
                }
            }
            #[inline]
            fn log2_stable(self) -> Self {
                crate::num::stable::log2(self)
            }
            #[inline]
            fn log10(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::log10(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::log10(self)
                }
            }
            #[inline]
            fn log10_stable(self) -> Self {
                crate::num::stable::log10(self)
            }
            #[inline]
            fn cbrt(self) -> Self {
                #[cfg(feature = "std")]
                {
                    self.as_array().map(|x| x.cbrt()).into()
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::cbrt(self)
                }
            }
            #[inline]
            fn cbrt_stable(self) -> Self {
                crate::num::stable::cbrt(self)
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                #[cfg(feature = "std")]
                {
                    // Scale by the larger magnitude so neither square can overflow or underflow.
                    let x = SimdFloat::abs(self);
                    let y = SimdFloat::abs(other);
                    let hi = SimdFloat::simd_max(x, y);
                    let lo = SimdFloat::simd_min(x, y);
                    let ratio = lo / hi;
                    let result = hi * StdFloat::sqrt(Simd::splat(1.0) + ratio * ratio);

                    // Restore `hypot` special cases.
                    let result = hi.simd_eq(Simd::splat(0.0)).select(Simd::splat(0.0), result);
                    let result = (SimdFloat::is_nan(self) | SimdFloat::is_nan(other))
                        .select(Self::NAN, result);
                    (SimdFloat::is_infinite(self) | SimdFloat::is_infinite(other))
                        .select(Self::INFINITY, result)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::hypot(self, other)
                }
            }
            #[inline]
            fn hypot_stable(self, other: Self) -> Self {
                crate::num::stable::hypot(self, other)
            }
            #[inline]
            fn sin(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::sin(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::sin(self)
                }
            }
            #[inline]
            fn sin_stable(self) -> Self {
                crate::num::stable::sin(self)
            }
            #[inline]
            fn cos(self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::cos(self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::cos(self)
                }
            }
            #[inline]
            fn cos_stable(self) -> Self {
                crate::num::stable::cos(self)
            }
            #[inline]
            fn tan(self) -> Self {
                #[cfg(feature = "std")]
                {
                    if <$real>::MANTISSA_DIGITS == f32::MANTISSA_DIGITS {
                        StdFloat::sin(self) / StdFloat::cos(self)
                    } else {
                        self.as_array().map(|x| x.tan()).into()
                    }
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::tan(self)
                }
            }
            #[inline]
            fn tan_stable(self) -> Self {
                crate::num::stable::tan(self)
            }
            #[inline]
            fn asin(self) -> Self {
                #[cfg(feature = "std")]
                {
                    self.as_array().map(|x| x.asin()).into()
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::asin(self)
                }
            }
            #[inline]
            fn asin_stable(self) -> Self {
                crate::num::stable::asin(self)
            }
            #[inline]
            fn acos(self) -> Self {
                #[cfg(feature = "std")]
                {
                    self.as_array().map(|x| x.acos()).into()
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::acos(self)
                }
            }
            #[inline]
            fn acos_stable(self) -> Self {
                crate::num::stable::acos(self)
            }
            #[inline]
            fn atan(self) -> Self {
                #[cfg(feature = "std")]
                {
                    self.as_array().map(|x| x.atan()).into()
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::atan(self)
                }
            }
            #[inline]
            fn atan_stable(self) -> Self {
                crate::num::stable::atan(self)
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                #[cfg(feature = "std")]
                {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = self[i].atan2(other[i]);
                    }
                    result
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::atan2(self, other)
                }
            }
            #[inline]
            fn atan2_stable(self, other: Self) -> Self {
                crate::num::stable::atan2(self, other)
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                #[cfg(feature = "std")]
                {
                    let mut sin = self;
                    let mut cos = self;
                    for i in 0..Self::LEN {
                        (sin[i], cos[i]) = self[i].sin_cos();
                    }
                    (sin, cos)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::sin_cos(self)
                }
            }
            #[inline]
            fn sin_cos_stable(self) -> (Self, Self) {
                crate::num::stable::sin_cos(self)
            }
            #[inline]
            fn sinh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    let x = SimdFloat::abs(self);
                    let root_e = StdFloat::exp(x * Simd::splat(0.5));
                    let half_e = root_e * (root_e * Simd::splat(0.5));
                    let result = half_e - Simd::splat(0.25) / half_e;
                    let small = StdFloat::sqrt(Simd::splat(<$real>::EPSILON));
                    let result = x.simd_lt(small).select(x, result);
                    SimdFloat::copysign(result, self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::sinh(self)
                }
            }
            #[inline]
            fn sinh_stable(self) -> Self {
                crate::num::stable::sinh(self)
            }
            #[inline]
            fn cosh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    let root_e = StdFloat::exp(SimdFloat::abs(self) * Simd::splat(0.5));
                    let half_e = root_e * (root_e * Simd::splat(0.5));
                    half_e + Simd::splat(0.25) / half_e
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::cosh(self)
                }
            }
            #[inline]
            fn cosh_stable(self) -> Self {
                crate::num::stable::cosh(self)
            }
            #[inline]
            fn tanh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    let x = SimdFloat::abs(self);
                    let e2 = StdFloat::exp(x + x);
                    let result = (e2 - Simd::splat(1.0)) / (e2 + Simd::splat(1.0));
                    let result = SimdFloat::is_infinite(e2).select(Simd::splat(1.0), result);
                    let small = StdFloat::sqrt(Simd::splat(<$real>::EPSILON));
                    let result = x.simd_lt(small).select(x, result);
                    SimdFloat::copysign(result, self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::tanh(self)
                }
            }
            #[inline]
            fn tanh_stable(self) -> Self {
                crate::num::stable::tanh(self)
            }
            #[inline]
            fn asinh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    if <$real>::MANTISSA_DIGITS == f32::MANTISSA_DIGITS {
                        return self.as_array().map(|x| x.asinh()).into();
                    }

                    let x = SimdFloat::abs(self);
                    let square_limit = StdFloat::sqrt(Simd::splat(<$real>::MAX));
                    let regular =
                        StdFloat::ln(x + StdFloat::sqrt(x * x + Simd::splat(1.0)));
                    let large = StdFloat::ln(x) + Simd::splat(<$real as RealConstants>::LN_2);
                    let result = x.simd_gt(square_limit).select(large, regular);
                    let small = StdFloat::sqrt(Simd::splat(<$real>::EPSILON));
                    let result = x.simd_lt(small).select(x, result);
                    SimdFloat::copysign(result, self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::asinh(self)
                }
            }
            #[inline]
            fn asinh_stable(self) -> Self {
                crate::num::stable::asinh(self)
            }
            #[inline]
            fn acosh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    if <$real>::MANTISSA_DIGITS == f32::MANTISSA_DIGITS {
                        return self.as_array().map(|x| x.acosh()).into();
                    }

                    let square_limit = StdFloat::sqrt(Simd::splat(<$real>::MAX));
                    let regular = StdFloat::ln(
                        self
                            + StdFloat::sqrt(
                                (self - Simd::splat(1.0)) * (self + Simd::splat(1.0)),
                            ),
                    );
                    let large = StdFloat::ln(self) + Simd::splat(<$real as RealConstants>::LN_2);
                    self.simd_gt(square_limit).select(large, regular)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::acosh(self)
                }
            }
            #[inline]
            fn acosh_stable(self) -> Self {
                crate::num::stable::acosh(self)
            }
            #[inline]
            fn atanh(self) -> Self {
                #[cfg(feature = "std")]
                {
                    let abs = SimdFloat::abs(self);
                    let result = StdFloat::ln(
                        (Simd::splat(1.0) + abs) / (Simd::splat(1.0) - abs),
                    ) * Simd::splat(0.5);
                    let small = StdFloat::sqrt(Simd::splat(<$real>::EPSILON));
                    let result = abs.simd_lt(small).select(abs, result);
                    SimdFloat::copysign(result, self)
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::atanh(self)
                }
            }
            #[inline]
            fn atanh_stable(self) -> Self {
                crate::num::stable::atanh(self)
            }
            #[inline]
            fn recip(self) -> Self {
                SimdFloat::recip(self)
            }
            #[inline]
            fn to_degrees(self) -> Self {
                SimdFloat::to_degrees(self)
            }
            #[inline]
            fn to_radians(self) -> Self {
                SimdFloat::to_radians(self)
            }
            #[inline]
            fn midpoint(self, other: Self) -> Self {
                // SIMD version of `$real::midpoint`.
                // Verified to be bit-identical to the scalar version.

                const HI: $real = <$real>::MAX / 2.0;

                let hi = Self::splat(HI);
                let half = Self::splat(0.5);

                let abs_a = Signed::abs(self);
                let abs_b = Signed::abs(other);
                let safe = abs_a.num_le(hi) & abs_b.num_le(hi);

                if safe.all() {
                    // Overflow is impossible
                    return (self + other) * half;
                }

                safe.select((self + other) * half, self * half + other * half)
            }
            #[inline]
            fn copysign(self, sign: Self) -> Self {
                SimdFloat::copysign(self, sign)
            }
        }
        )+
    };
}

impl_real_simd!(f32, f64);

macro_rules! impl_float_simd {
    ($float:tt, $uint:ty, $int:ty) => {
        impl<const N: usize> Float for Simd<$float, N> {
            type Bits = Simd<$uint, N>;

            const RADIX: u32 = $float::RADIX;
            const MANTISSA_DIGITS: u32 = $float::MANTISSA_DIGITS;
            const DIGITS: u32 = $float::DIGITS;
            const EPSILON: Self = Self::splat($float::EPSILON);
            const MIN_POSITIVE: Self = Self::splat($float::MIN_POSITIVE);
            const MIN_EXP: i32 = $float::MIN_EXP;
            const MAX_EXP: i32 = $float::MAX_EXP;
            const MIN_10_EXP: i32 = $float::MIN_10_EXP;
            const MAX_10_EXP: i32 = $float::MAX_10_EXP;
            const NAN: Self = Self::splat($float::NAN);
            const INFINITY: Self = Self::splat($float::INFINITY);
            const NEG_INFINITY: Self = Self::splat($float::NEG_INFINITY);

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                #[cfg(feature = "std")]
                {
                    StdFloat::mul_add(self, a, b)
                }
                #[cfg(not(feature = "std"))]
                {
                    unsafe { core::intrinsics::simd::simd_fma(self, a, b) }
                }
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                #[cfg(feature = "std")]
                {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = self[i].powf(n[i]);
                    }
                    result
                }
                #[cfg(not(feature = "std"))]
                {
                    crate::num::stable::powf(self, n)
                }
            }
            #[inline]
            fn powf_stable(self, n: Self) -> Self {
                crate::num::stable::powf(self, n)
            }
            #[inline]
            fn is_nan(self) -> Self::Bool {
                SimdFloat::is_nan(self)
            }
            #[inline]
            fn is_infinite(self) -> Self::Bool {
                SimdFloat::is_infinite(self)
            }
            #[inline]
            fn is_finite(self) -> Self::Bool {
                SimdFloat::is_finite(self)
            }
            #[inline]
            fn is_subnormal(self) -> Self::Bool {
                SimdFloat::is_subnormal(self)
            }
            #[inline]
            fn is_normal(self) -> Self::Bool {
                SimdFloat::is_normal(self)
            }
            #[inline]
            fn is_sign_positive(self) -> Self::Bool {
                SimdFloat::is_sign_positive(self)
            }
            #[inline]
            fn is_sign_negative(self) -> Self::Bool {
                SimdFloat::is_sign_negative(self)
            }
            #[inline]
            fn next_up(self) -> Self {
                let bits: Simd<$int, N> = SimdUint::cast(SimdFloat::to_bits(self));
                let magnitude = bits & Simd::splat(<$int>::MAX);
                let is_zero = magnitude.simd_eq(Simd::splat(0));
                let is_negative = bits.simd_lt(Simd::splat(0));
                let adjacent = is_negative.select(bits - Simd::splat(1), bits + Simd::splat(1));
                let candidate =
                    SimdFloat::from_bits(SimdInt::cast(is_zero.select(Simd::splat(1), adjacent)));
                let unchanged =
                    SimdFloat::is_nan(self) | self.simd_eq(Simd::splat(<$float>::INFINITY));
                unchanged.select(self, candidate)
            }
            #[inline]
            fn next_down(self) -> Self {
                let bits: Simd<$int, N> = SimdUint::cast(SimdFloat::to_bits(self));
                let sign = Simd::splat(<$int>::MIN);
                let magnitude = bits & Simd::splat(<$int>::MAX);
                let is_zero = magnitude.simd_eq(Simd::splat(0));
                let is_negative = bits.simd_lt(Simd::splat(0));
                let adjacent = is_negative.select(bits + Simd::splat(1), bits - Simd::splat(1));
                let negative_min = sign | Simd::splat(1);
                let candidate =
                    SimdFloat::from_bits(SimdInt::cast(is_zero.select(negative_min, adjacent)));
                let unchanged =
                    SimdFloat::is_nan(self) | self.simd_eq(Simd::splat(<$float>::NEG_INFINITY));
                unchanged.select(self, candidate)
            }
            #[inline]
            fn from_bits(bits: Self::Bits) -> Self {
                SimdFloat::from_bits(bits)
            }
            #[inline]
            fn to_bits(self) -> Self::Bits {
                SimdFloat::to_bits(self)
            }
            #[inline]
            fn from_int(int: Simd<$int, N>) -> Self {
                SimdInt::cast(int)
            }
            #[inline]
            fn to_int(self) -> Simd<$int, N> {
                SimdFloat::cast(self)
            }
        }
    };
}

impl_float_simd!(f32, u32, i32);
impl_float_simd!(f64, u64, i64);

use crate::num::{Float, Real, RealConstants};
use core::simd::{Simd, SimdElement, num::SimdFloat};
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
            #[inline]
            fn floor(self) -> Self {
                StdFloat::floor(self)
            }
            #[inline]
            fn ceil(self) -> Self {
                StdFloat::ceil(self)
            }
            #[inline]
            fn round(self) -> Self {
                StdFloat::round(self)
            }
            #[inline]
            fn trunc(self) -> Self {
                StdFloat::trunc(self)
            }
            #[inline]
            fn fract(self) -> Self {
                StdFloat::fract(self)
            }
            #[inline]
            fn sqrt(self) -> Self {
                StdFloat::sqrt(self)
            }
            #[inline]
            fn exp(self) -> Self {
                StdFloat::exp(self)
            }
            #[inline]
            fn exp2(self) -> Self {
                StdFloat::exp2(self)
            }
            #[inline]
            fn ln(self) -> Self {
                StdFloat::ln(self)
            }
            #[inline]
            fn log(self, base: Self) -> Self {
                StdFloat::log(self, base)
            }
            #[inline]
            fn log2(self) -> Self {
                StdFloat::log2(self)
            }
            #[inline]
            fn log10(self) -> Self {
                StdFloat::log10(self)
            }
            #[inline]
            fn cbrt(self) -> Self {
                self.as_array().map(|x| x.cbrt()).into()
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].hypot(other[i]);
                }
                result
            }
            #[inline]
            fn sin(self) -> Self {
                StdFloat::sin(self)
            }
            #[inline]
            fn cos(self) -> Self {
                StdFloat::cos(self)
            }
            #[inline]
            fn tan(self) -> Self {
                self.as_array().map(|x| x.tan()).into()
            }
            #[inline]
            fn asin(self) -> Self {
                self.as_array().map(|x| x.asin()).into()
            }
            #[inline]
            fn acos(self) -> Self {
                self.as_array().map(|x| x.acos()).into()
            }
            #[inline]
            fn atan(self) -> Self {
                self.as_array().map(|x| x.atan()).into()
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].atan2(other[i]);
                }
                result
            }
            #[inline]
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
            #[inline]
            fn sinh(self) -> Self {
                self.as_array().map(|x| x.sinh()).into()
            }
            #[inline]
            fn cosh(self) -> Self {
                self.as_array().map(|x| x.cosh()).into()
            }
            #[inline]
            fn tanh(self) -> Self {
                self.as_array().map(|x| x.tanh()).into()
            }
            #[inline]
            fn asinh(self) -> Self {
                self.as_array().map(|x| x.asinh()).into()
            }
            #[inline]
            fn acosh(self) -> Self {
                self.as_array().map(|x| x.acosh()).into()
            }
            #[inline]
            fn atanh(self) -> Self {
                self.as_array().map(|x| x.atanh()).into()
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
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].midpoint(other[i]);
                }
                result
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
    ($float:tt, $int:ty) => {
        impl<const N: usize> Float for Simd<$float, N> {
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
                StdFloat::mul_add(self, a, b)
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                let mut result = self;
                for i in 0..Self::LEN {
                    result[i] = result[i].powf(n[i]);
                }
                result
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
                self.as_array().map(|x| x.next_up()).into()
            }
            #[inline]
            fn next_down(self) -> Self {
                self.as_array().map(|x| x.next_down()).into()
            }
        }
    };
}

impl_float_simd!(f32, i32);
impl_float_simd!(f64, i64);

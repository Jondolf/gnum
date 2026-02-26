use crate::float::{Float, FloatClassify};
use core::num::FpCategory;

impl Float for f32 {
    type Bits = u32;
    type Int = i32;
    type Sign = i32;

    const RADIX: u32 = f32::RADIX;
    const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    const DIGITS: u32 = f32::DIGITS;
    const EPSILON: Self = f32::EPSILON;
    const MIN: Self = f32::MIN;
    const MIN_POSITIVE: Self = f32::MIN_POSITIVE;
    const MAX: Self = f32::MAX;
    const MIN_EXP: i32 = f32::MIN_EXP;
    const MAX_EXP: i32 = f32::MAX_EXP;
    const MIN_10_EXP: i32 = f32::MIN_10_EXP;
    const MAX_10_EXP: i32 = f32::MAX_10_EXP;
    const NAN: Self = f32::NAN;
    const INFINITY: Self = f32::INFINITY;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;

    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
    fn fract(self) -> Self {
        self.fract()
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
    fn div_euclid(self, rhs: Self) -> Self {
        self.div_euclid(rhs)
    }
    fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn exp2(self) -> Self {
        self.exp2()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log(self, base: Self) -> Self {
        self.log(base)
    }
    fn log2(self) -> Self {
        self.log2()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn cbrt(self) -> Self {
        self.cbrt()
    }
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn exp_m1(self) -> Self {
        self.exp_m1()
    }
    fn ln_1p(self) -> Self {
        self.ln_1p()
    }
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn asinh(self) -> Self {
        self.asinh()
    }
    fn acosh(self) -> Self {
        self.acosh()
    }
    fn atanh(self) -> Self {
        self.atanh()
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_subnormal(self) -> bool {
        self.is_subnormal()
    }
    fn is_normal(self) -> bool {
        self.is_normal()
    }
    fn is_sign_positive(self) -> bool {
        self.is_sign_positive()
    }
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }
    fn next_up(self) -> Self {
        self.next_up()
    }
    fn next_down(self) -> Self {
        self.next_down()
    }
    fn recip(self) -> Self {
        self.recip()
    }
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }
    fn to_radians(self) -> Self {
        self.to_radians()
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn midpoint(self, other: Self) -> Self {
        self.midpoint(other)
    }
    fn to_bits(self) -> u32 {
        self.to_bits()
    }
    fn from_bits(bits: u32) -> Self {
        Self::from_bits(bits)
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn signum(self) -> Self {
        self.signum()
    }
    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
}

impl Float for f64 {
    type Bits = u64;
    type Int = i32;
    type Sign = i32;

    const RADIX: u32 = f64::RADIX;
    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    const DIGITS: u32 = f64::DIGITS;
    const EPSILON: Self = f64::EPSILON;
    const MIN: Self = f64::MIN;
    const MIN_POSITIVE: Self = f64::MIN_POSITIVE;
    const MAX: Self = f64::MAX;
    const MIN_EXP: i32 = f64::MIN_EXP;
    const MAX_EXP: i32 = f64::MAX_EXP;
    const MIN_10_EXP: i32 = f64::MIN_10_EXP;
    const MAX_10_EXP: i32 = f64::MAX_10_EXP;
    const NAN: Self = f64::NAN;
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
    fn fract(self) -> Self {
        self.fract()
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
    fn div_euclid(self, rhs: Self) -> Self {
        self.div_euclid(rhs)
    }
    fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn exp2(self) -> Self {
        self.exp2()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log(self, base: Self) -> Self {
        self.log(base)
    }
    fn log2(self) -> Self {
        self.log2()
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn cbrt(self) -> Self {
        self.cbrt()
    }
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn exp_m1(self) -> Self {
        self.exp_m1()
    }
    fn ln_1p(self) -> Self {
        self.ln_1p()
    }
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn asinh(self) -> Self {
        self.asinh()
    }
    fn acosh(self) -> Self {
        self.acosh()
    }
    fn atanh(self) -> Self {
        self.atanh()
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_subnormal(self) -> bool {
        self.is_subnormal()
    }
    fn is_normal(self) -> bool {
        self.is_normal()
    }
    fn is_sign_positive(self) -> bool {
        self.is_sign_positive()
    }
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }
    fn next_up(self) -> Self {
        self.next_up()
    }
    fn next_down(self) -> Self {
        self.next_down()
    }
    fn recip(self) -> Self {
        self.recip()
    }
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }
    fn to_radians(self) -> Self {
        self.to_radians()
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn midpoint(self, other: Self) -> Self {
        self.midpoint(other)
    }
    fn to_bits(self) -> Self::Bits {
        self.to_bits()
    }
    fn from_bits(bits: Self::Bits) -> Self {
        Self::from_bits(bits)
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn signum(self) -> Self {
        self.signum()
    }
    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
}

impl FloatClassify for f32 {
    fn classify(self) -> FpCategory {
        self.classify()
    }
}

impl FloatClassify for f64 {
    fn classify(self) -> FpCategory {
        self.classify()
    }
}

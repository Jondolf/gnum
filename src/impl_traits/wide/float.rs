use super::round::RoundOps;
use crate::cmp::NumOrd;
use crate::num::{Float, Real, RealConstants, Signed};
use wide::bytemuck;
use wide::*;

macro_rules! impl_real_constants {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl RealConstants for $simd {
                const HALF: Self = Self::splat(<$elem as RealConstants>::HALF);
                const E: Self = Self::splat(<$elem as RealConstants>::E);
                const EULER_GAMMA: Self = Self::splat(<$elem as RealConstants>::EULER_GAMMA);
                const FRAC_1_PI: Self = Self::splat(<$elem as RealConstants>::FRAC_1_PI);
                const FRAC_1_SQRT_2: Self = Self::splat(<$elem as RealConstants>::FRAC_1_SQRT_2);
                const FRAC_2_PI: Self = Self::splat(<$elem as RealConstants>::FRAC_2_PI);
                const FRAC_2_SQRT_PI: Self = Self::splat(<$elem as RealConstants>::FRAC_2_SQRT_PI);
                const FRAC_PI_2: Self = Self::splat(<$elem as RealConstants>::FRAC_PI_2);
                const FRAC_PI_3: Self = Self::splat(<$elem as RealConstants>::FRAC_PI_3);
                const FRAC_PI_4: Self = Self::splat(<$elem as RealConstants>::FRAC_PI_4);
                const FRAC_PI_6: Self = Self::splat(<$elem as RealConstants>::FRAC_PI_6);
                const FRAC_PI_8: Self = Self::splat(<$elem as RealConstants>::FRAC_PI_8);
                const GOLDEN_RATIO: Self = Self::splat(<$elem as RealConstants>::GOLDEN_RATIO);
                const LN_2: Self = Self::splat(<$elem as RealConstants>::LN_2);
                const LN_10: Self = Self::splat(<$elem as RealConstants>::LN_10);
                const LOG2_10: Self = Self::splat(<$elem as RealConstants>::LOG2_10);
                const LOG2_E: Self = Self::splat(<$elem as RealConstants>::LOG2_E);
                const LOG10_2: Self = Self::splat(<$elem as RealConstants>::LOG10_2);
                const LOG10_E: Self = Self::splat(<$elem as RealConstants>::LOG10_E);
                const PI: Self = Self::splat(<$elem as RealConstants>::PI);
                const SQRT_2: Self = Self::splat(<$elem as RealConstants>::SQRT_2);
                const TAU: Self = Self::splat(<$elem as RealConstants>::TAU);
            }
        )*
    };
}

impl_real_constants!(
    f32x4 => f32, f32x8 => f32, f32x16 => f32,
    f64x2 => f64, f64x4 => f64, f64x8 => f64,
);

macro_rules! impl_real {
    ($($simd:ident => $elem:ty, $i32:ident);* $(;)?) => {
        $(
            impl Real for $simd {
                type I32 = $i32;

                #[inline]
                fn from_f32(n: f32) -> Self {
                    Self::splat(n as $elem)
                }
                #[inline]
                fn from_f64(n: f64) -> Self {
                    Self::splat(n as $elem)
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
                    self.fract()
                }
                #[inline]
                fn sqrt(self) -> Self {
                    self.sqrt()
                }
                #[inline]
                fn exp(self) -> Self {
                    self.exp()
                }
                #[inline]
                fn exp_stable(self) -> Self {
                    crate::num::stable::exp(self)
                }
                #[inline]
                fn exp2(self) -> Self {
                    self.exp2()
                }
                #[inline]
                fn exp2_stable(self) -> Self {
                    crate::num::stable::exp2(self)
                }
                #[inline]
                fn log(self, base: Self) -> Self {
                    <Self as Real>::ln(self) / <Self as Real>::ln(base)
                }
                #[inline]
                fn log_stable(self, base: Self) -> Self {
                    crate::num::stable::log(self, base)
                }
                #[inline]
                fn ln(self) -> Self {
                    let result = self.ln();

                    // Handle subnormals properly
                    let min_positive = Self::splat(<$elem>::MIN_POSITIVE);
                    if self.simd_ge(min_positive).all() {
                        return result;
                    }
                    let subnormal = self.simd_gt(Self::ZERO) & self.simd_lt(min_positive);
                    if !subnormal.any() {
                        return result;
                    }
                    let (scale, correction) = if <$elem>::MANTISSA_DIGITS == f32::MANTISSA_DIGITS {
                        (16_777_216.0 as $elem, 24.0 as $elem)
                    } else {
                        (18_014_398_509_481_984.0 as $elem, 54.0 as $elem)
                    };
                    let normalized = (self * Self::splat(scale)).ln()
                        - Self::splat(correction * <$elem as RealConstants>::LN_2);
                    subnormal.select(normalized, result)
                }
                #[inline]
                fn ln_stable(self) -> Self {
                    crate::num::stable::ln(self)
                }
                #[inline]
                fn log2(self) -> Self {
                    <Self as Real>::ln(self) * Self::LOG2_E
                }
                #[inline]
                fn log2_stable(self) -> Self {
                    crate::num::stable::log2(self)
                }
                #[inline]
                fn log10(self) -> Self {
                    <Self as Real>::ln(self) * Self::LOG10_E
                }
                #[inline]
                fn log10_stable(self) -> Self {
                    crate::num::stable::log10(self)
                }
                #[inline]
                fn cbrt(self) -> Self {
                    self.cbrt()
                }
                #[inline]
                fn cbrt_stable(self) -> Self {
                    crate::num::stable::cbrt(self)
                }
                #[inline]
                fn hypot(self, other: Self) -> Self {
                    // Scale by the larger magnitude so neither square can overflow or underflow.
                    let x = self.abs();
                    let y = other.abs();
                    let swap = x.simd_lt(y);
                    let hi = swap.select(y, x);
                    let lo = swap.select(x, y);
                    let ratio = lo / hi;
                    let result = hi * (Self::ONE + ratio * ratio).sqrt();

                    // Restore `hypot` special cases.
                    let result = hi.simd_eq(Self::ZERO).select(Self::ZERO, result);
                    let result = (self.is_nan() | other.is_nan()).select(Self::NAN, result);
                    (self.is_inf() | other.is_inf()).select(Self::INFINITY, result)
                }
                #[inline]
                fn hypot_stable(self, other: Self) -> Self {
                    crate::num::stable::hypot(self, other)
                }
                #[inline]
                fn sin(self) -> Self {
                    self.sin()
                }
                #[inline]
                fn sin_stable(self) -> Self {
                    crate::num::stable::sin(self)
                }
                #[inline]
                fn cos(self) -> Self {
                    self.cos()
                }
                #[inline]
                fn cos_stable(self) -> Self {
                    crate::num::stable::cos(self)
                }
                #[inline]
                fn tan(self) -> Self {
                    self.tan()
                }
                #[inline]
                fn tan_stable(self) -> Self {
                    crate::num::stable::tan(self)
                }
                #[inline]
                fn asin(self) -> Self {
                    self.asin()
                }
                #[inline]
                fn asin_stable(self) -> Self {
                    crate::num::stable::asin(self)
                }
                #[inline]
                fn acos(self) -> Self {
                    self.acos()
                }
                #[inline]
                fn acos_stable(self) -> Self {
                    crate::num::stable::acos(self)
                }
                #[inline]
                fn atan(self) -> Self {
                    self.atan()
                }
                #[inline]
                fn atan_stable(self) -> Self {
                    crate::num::stable::atan(self)
                }
                #[inline]
                fn atan2(self, other: Self) -> Self {
                    self.atan2(other)
                }
                #[inline]
                fn atan2_stable(self, other: Self) -> Self {
                    crate::num::stable::atan2(self, other)
                }
                #[inline]
                fn sin_cos(self) -> (Self, Self) {
                    self.sin_cos()
                }
                #[inline]
                fn sin_cos_stable(self) -> (Self, Self) {
                    crate::num::stable::sin_cos(self)
                }
                #[inline]
                fn sinh(self) -> Self {
                    self.sinh()
                }
                #[inline]
                fn sinh_stable(self) -> Self {
                    crate::num::stable::sinh(self)
                }
                #[inline]
                fn cosh(self) -> Self {
                    self.cosh()
                }
                #[inline]
                fn cosh_stable(self) -> Self {
                    crate::num::stable::cosh(self)
                }
                #[inline]
                fn tanh(self) -> Self {
                    let x = self.abs();
                    let e2 = (x + x).exp();
                    let result = (e2 - Self::ONE) / (e2 + Self::ONE);
                    let result = e2.is_inf().select(Self::ONE, result);
                    let small = Self::splat(<$elem>::EPSILON.sqrt());
                    let result = x.simd_lt(small).select(x, result);
                    result.copysign(self)
                }
                #[inline]
                fn tanh_stable(self) -> Self {
                    crate::num::stable::tanh(self)
                }
                #[inline]
                fn asinh(self) -> Self {
                    let x = self.abs();
                    let square_limit = Self::splat(<$elem>::MAX.sqrt());
                    let regular = (x + (x * x + Self::ONE).sqrt()).ln();
                    let large = <Self as Real>::ln(x) + Self::LN_2;
                    let result = x.simd_gt(square_limit).select(large, regular);
                    let small = Self::splat(<$elem>::EPSILON.sqrt());
                    let result = x.simd_lt(small).select(x, result);
                    result.copysign(self)
                }
                #[inline]
                fn asinh_stable(self) -> Self {
                    crate::num::stable::asinh(self)
                }
                #[inline]
                fn acosh(self) -> Self {
                    let square_limit = Self::splat(<$elem>::MAX.sqrt());
                    let regular = (self + ((self - Self::ONE) * (self + Self::ONE)).sqrt()).ln();
                    let large = <Self as Real>::ln(self) + Self::LN_2;
                    self.simd_gt(square_limit).select(large, regular)
                }
                #[inline]
                fn acosh_stable(self) -> Self {
                    crate::num::stable::acosh(self)
                }
                #[inline]
                fn atanh(self) -> Self {
                    let x = self.abs();
                    let result = ((Self::ONE + x) / (Self::ONE - x)).ln() * Self::HALF;
                    let small = Self::splat(<$elem>::EPSILON.sqrt());
                    let result = x.simd_lt(small).select(x, result);
                    result.copysign(self)
                }
                #[inline]
                fn atanh_stable(self) -> Self {
                    crate::num::stable::atanh(self)
                }
                #[inline]
                fn recip(self) -> Self {
                    Self::ONE / self
                }
                #[inline]
                fn to_degrees(self) -> Self {
                    self * Self::splat((1.0 as $elem).to_degrees())
                }
                #[inline]
                fn to_radians(self) -> Self {
                    self * Self::splat((1.0 as $elem).to_radians())
                }
                #[inline]
                fn midpoint(self, other: Self) -> Self {
                    // SIMD version of `$real::midpoint`.
                    // Verified to be bit-identical to the scalar version.
                    const HI: $elem = <$elem>::MAX / 2.0;

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
                    self.copysign(sign)
                }
            }
        )*
    };
}

impl_real!(
    f32x4 => f32, i32x4; f32x8 => f32, i32x8; f32x16 => f32, i32x16;
    f64x2 => f64, i64x2; f64x4 => f64, i64x4; f64x8 => f64, i64x8;
);

macro_rules! impl_float {
    ($($simd:ident => $elem:ty, $uint:ident, $int:ident, $sielem:ty, $true:expr, $false:expr);* $(;)?) => {
        $(
            impl Float for $simd {
                type Bits = $uint;

                const RADIX: u32 = <$elem>::RADIX;
                const MANTISSA_DIGITS: u32 = <$elem>::MANTISSA_DIGITS;
                const DIGITS: u32 = <$elem>::DIGITS;
                const EPSILON: Self = Self::splat(<$elem>::EPSILON);
                const MIN_POSITIVE: Self = Self::splat(<$elem>::MIN_POSITIVE);
                const MIN_EXP: i32 = <$elem>::MIN_EXP;
                const MAX_EXP: i32 = <$elem>::MAX_EXP;
                const MIN_10_EXP: i32 = <$elem>::MIN_10_EXP;
                const MAX_10_EXP: i32 = <$elem>::MAX_10_EXP;
                const NAN: Self = Self::splat(<$elem>::NAN);
                const INFINITY: Self = Self::splat(<$elem>::INFINITY);
                const NEG_INFINITY: Self = Self::splat(<$elem>::NEG_INFINITY);

                #[inline]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    #[cfg(any(
                        all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "fma"),
                        all(target_arch = "aarch64", target_feature = "neon"),
                    ))]
                    {
                        return self.mul_add(a, b);
                    }

                    // Fused per lane to match `f32::mul_add` exactly.
                    #[cfg(not(any(
                        all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "fma"),
                        all(target_arch = "aarch64", target_feature = "neon"),
                    )))]
                    {
                        let aa = a.to_array();
                        let bb = b.to_array();
                        let mut i = 0usize;
                        let r = self.to_array().map(|x: $elem| {
                            let v = x.mul_add(aa[i], bb[i]);
                            i += 1;
                            v
                        });
                        Self::new(r)
                    }
                }
                #[inline]
                fn powf(self, n: Self) -> Self {
                    let result = self.powf_simd(n);

                    // Handle subnormals and NaN properly
                    let min_positive = Self::splat(<$elem>::MIN_POSITIVE);
                    let normal_or_zero = self.abs().simd_ge(min_positive)
                        | self.simd_eq(Self::ZERO);
                    let ordinary = self.is_finite() & n.is_finite() & normal_or_zero;
                    if ordinary.all() {
                        return result;
                    }

                    let xx = self.to_array();
                    let nn = n.to_array();
                    let mut lanes = result.to_array();
                    for i in 0..lanes.len() {
                        let x = xx[i];
                        let y = nn[i];
                        if !x.is_finite()
                            || !y.is_finite()
                            || (x != 0.0 && x.abs() < <$elem>::MIN_POSITIVE)
                        {
                            lanes[i] = x.powf(y);
                        }
                    }
                    Self::new(lanes)
                }
                #[inline]
                fn powf_stable(self, n: Self) -> Self {
                    crate::num::stable::powf(self, n)
                }
                #[inline]
                fn is_nan(self) -> Self::Bool {
                    self.is_nan()
                }
                #[inline]
                fn is_infinite(self) -> Self::Bool {
                    self.is_inf()
                }
                #[inline(always)]
                fn is_finite(self) -> Self::Bool {
                    self.abs().simd_lt(Self::INFINITY)
                }
                #[inline]
                fn is_subnormal(self) -> Self::Bool {
                    const EXPONENT: $sielem = <$elem>::INFINITY.to_bits() as $sielem;
                    const SIGN: $sielem = (-0.0 as $elem).to_bits() as $sielem;
                    const MANTISSA: $sielem = !(EXPONENT | SIGN);

                    let bits: $int = bytemuck::cast(self);
                    let exponent = bits & <$int>::splat(EXPONENT);
                    let mantissa = bits & <$int>::splat(MANTISSA);
                    bytemuck::cast(
                        exponent.simd_eq(<$int>::ZERO) & mantissa.simd_ne(<$int>::ZERO),
                    )
                }
                #[inline]
                fn is_normal(self) -> Self::Bool {
                    const EXPONENT: $sielem = <$elem>::INFINITY.to_bits() as $sielem;

                    let bits: $int = bytemuck::cast(self);
                    let exponent = bits & <$int>::splat(EXPONENT);
                    bytemuck::cast(
                        exponent.simd_ne(<$int>::ZERO)
                            & exponent.simd_ne(<$int>::splat(EXPONENT)),
                    )
                }
                #[inline]
                fn is_sign_positive(self) -> Self::Bool {
                    self.is_sign_positive()
                }
                #[inline]
                fn is_sign_negative(self) -> Self::Bool {
                    self.is_sign_negative()
                }
                #[inline]
                fn next_up(self) -> Self {
                    const SIGN: $sielem = (-0.0 as $elem).to_bits() as $sielem;

                    let bits: $int = bytemuck::cast(self);
                    let magnitude = bits & <$int>::splat(!SIGN);
                    let is_zero = magnitude.simd_eq(<$int>::ZERO);
                    let is_negative = bits.simd_lt(<$int>::ZERO);
                    let adjacent = is_negative.select(bits - <$int>::ONE, bits + <$int>::ONE);
                    let candidate: Self = bytemuck::cast(is_zero.select(<$int>::ONE, adjacent));
                    let unchanged = self.is_nan() | self.simd_eq(Self::INFINITY);
                    unchanged.select(self, candidate)
                }
                #[inline]
                fn next_down(self) -> Self {
                    const SIGN: $sielem = (-0.0 as $elem).to_bits() as $sielem;

                    let bits: $int = bytemuck::cast(self);
                    let magnitude = bits & <$int>::splat(!SIGN);
                    let is_zero = magnitude.simd_eq(<$int>::ZERO);
                    let is_negative = bits.simd_lt(<$int>::ZERO);
                    let adjacent = is_negative.select(bits + <$int>::ONE, bits - <$int>::ONE);
                    let negative_min = <$int>::splat(SIGN | 1);
                    let candidate: Self = bytemuck::cast(is_zero.select(negative_min, adjacent));
                    let unchanged = self.is_nan() | self.simd_eq(Self::NEG_INFINITY);
                    unchanged.select(self, candidate)
                }
                #[inline]
                fn from_bits(bits: Self::Bits) -> Self {
                    bytemuck::cast(bits)
                }
                #[inline]
                fn to_bits(self) -> Self::Bits {
                    bytemuck::cast(self)
                }
                #[inline]
                fn from_int(int: $int) -> Self {
                    int.round_float()
                }
                #[inline]
                fn to_int(self) -> $int {
                    self.trunc_int()
                }
            }
        )*
    };
}

impl_float!(
    f32x4 => f32, u32x4, i32x4, i32, f32::from_bits(u32::MAX), 0.0;
    f32x8 => f32, u32x8, i32x8, i32, f32::from_bits(u32::MAX), 0.0;
    f32x16 => f32, u32x16, i32x16, i32, f32::from_bits(u32::MAX), 0.0;
    f64x2 => f64, u64x2, i64x2, i64, f64::from_bits(u64::MAX), 0.0;
    f64x4 => f64, u64x4, i64x4, i64, f64::from_bits(u64::MAX), 0.0;
    f64x8 => f64, u64x8, i64x8, i64, f64::from_bits(u64::MAX), 0.0;
);

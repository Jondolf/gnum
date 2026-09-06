//! Deterministic, portable approximations for the `*_stable` transcendental operations.
//!
//! Both the scalar and SIMD [`Real`] and [`Float`] implementations delegate to the same
//! generic code, so a `*_stable` method is guaranteed to return identical results
//! across scalar and vectorized types of the same precision (ex: `f32` and `f32x4`),
//! and across all platforms on IEEE-754-compliant hardware.
//!
//! The reductions and minimax polynomials are adapted from [SLEEF] and [cephes].
//! They are near-correctly-rounded for [`f32`] and currently accurate to roughly single
//! precision for [`f64`]. Code that needs full double precision (and does not need
//! determinism) should use the non-`*_stable` methods, which map to the platform's
//! native routines. Accurate double-precision implementations may be added in the future.
//!
//! The `*_stable` methods are typically slower than the non-`*_stable` methods
//! for scalar `f32` and `f64`, but competitive or even faster for SIMD types.
//! This can vary by operation and by platform.
//!
//! [SLEEF]: https://sleef.org/
//! [cephes]: https://www.netlib.org/cephes/

#![expect(
    clippy::excessive_precision,
    reason = "constants transcribed from SLEEF and cephes"
)]

use crate::{
    cmp::NumOrd,
    num::{Float, Int, Real},
    simd::Select,
};

/// Negates `x` when the integer nearest to `q` is odd, applying the odd-quadrant sign.
#[inline(always)]
fn negate_if_odd<T: Float>(x: T, q: T) -> T {
    let q_odd = (q * T::HALF).fract().num_ne(T::ZERO);
    q_odd.select(-x, x)
}

// The high and low parts of ln(2) such that `LN_2_UPPER + LN_2_LOWER == ln(2)`
// to beyond f32 precision. Used for the Cody–Waite reduction in `exp`/`expm1`.
//
// SLEEF calls these L2U and L2L:
// https://github.com/shibatch/sleef/blob/7623d6cfa2712462880fa63a4d0f0b5f775d1a83/src/common/misc.h#L141-L142
const LN_2_UPPER: f64 = 0.693145751953125;
const LN_2_LOWER: f64 = 1.428606765330187045e-06;

// == Helpers ==

/// A float constant, to avoid spamming `T::from_f64` everywhere :3
#[inline(always)]
fn k<T: Real>(value: f64) -> T {
    T::from_f64(value)
}

/// The signed integer representation used for the exponent arithmetic.
type Bits<T> = <<T as Float>::Bits as Int>::Signed;

/// Broadcasts a small integer constant into the signed integer representation.
#[inline(always)]
fn ibits<T: Float>(value: i32) -> Bits<T> {
    T::to_int(T::from_f64(value as f64))
}

/// Number of explicit mantissa bits (23 for `f32`, 52 for `f64`).
#[inline(always)]
fn mantissa_bits<T: Float>() -> Bits<T> {
    ibits::<T>(T::MANTISSA_DIGITS as i32 - 1)
}

/// Exponent bias (127 for `f32`, 1023 for `f64`).
#[inline(always)]
fn exp_bias<T: Float>() -> Bits<T> {
    ibits::<T>(T::MAX_EXP - 1)
}

/// Number of exponent bits (8 for `f32`, 11 for `f64`).
#[inline(always)]
fn exponent_bits<T: Float>() -> Bits<T> {
    ibits::<T>((<T::Bits as Int>::BITS - T::MANTISSA_DIGITS) as i32)
}

/// Largest input for which `exp` is finite. Above this, `exp` saturates to +∞.
#[inline(always)]
fn exp_hi<T: Float>() -> T {
    if <T::Bits as Int>::BITS == 32 {
        T::from_f32(88.722839)
    } else {
        T::from_f64(709.782712893384)
    }
}

/// Smallest input for which `exp` is nonzero. Below this, `exp` flushes to 0.
#[inline(always)]
fn exp_lo<T: Float>() -> T {
    if <T::Bits as Int>::BITS == 32 {
        T::from_f32(-87.336544)
    } else {
        T::from_f64(-708.3964185322641)
    }
}

/// Scale that lifts any subnormal value into the normal range.
#[inline(always)]
fn subnormal_scale<T: Float>() -> T {
    if <T::Bits as Int>::BITS == 32 {
        k::<T>(16777216.0) // 2^24
    } else {
        k::<T>(18014398509481984.0) // 2^54
    }
}

/// Base-2 logarithm of [`subnormal_scale`], to put back into an exponent.
#[inline(always)]
fn subnormal_shift<T: Float>() -> T {
    if <T::Bits as Int>::BITS == 32 {
        k::<T>(24.0)
    } else {
        k::<T>(54.0)
    }
}

/// Cube root of [`subnormal_scale`], to undo it after a [`cbrt`].
#[inline(always)]
fn subnormal_cbrt_unscale<T: Float>() -> T {
    if <T::Bits as Int>::BITS == 32 {
        k::<T>(0.00390625) // 2^-8
    } else {
        k::<T>(3.814697265625e-06) // 2^-18
    }
}

/// Computes `2^n` for an integer `n`.
///
/// Valid while `n + bias` stays within the exponent range.
/// Callers guard out-of-range inputs separately.
#[inline(always)]
fn pow2i<T: Float>(n: Bits<T>) -> T {
    let biased = (n + exp_bias::<T>()) << mantissa_bits::<T>();
    // The assembled bit pattern is reinterpreted through the unsigned `Bits` type.
    T::from_bits(biased.cast_unsigned())
}

/// Reads the exponent `n` from `x = m * 2^n` with `m` in `[1, 2)`.
/// This is equivalent to `floor(log2(x))` for a positive `x`.
///
/// The exponent is extracted straight out of the float's bit layout.
/// Callers are responsible for ensuring `x` is normal (not subnormal).
#[inline(always)]
fn ilogb<T: Float>(x: T) -> Bits<T> {
    let exp_mask = (ibits::<T>(1) << exponent_bits::<T>()) - ibits::<T>(1);
    // `x` is positive here, so the sign bit is clear and the shift stays exact.
    ((x.to_bits().cast_signed() >> mantissa_bits::<T>()) & exp_mask) - exp_bias::<T>()
}

/// Returns `x * 2^-e`.
///
/// `x` must be non-negative, and `e` must be close enough to its exponent that
/// the result stays normal, which holds when `e` is the exponent `x` was split on.
#[inline(always)]
fn scale_exp2<T: Float>(x: T, e: Bits<T>) -> T {
    let shifted = x.to_bits().cast_signed() - (e << mantissa_bits::<T>());
    T::from_bits(shifted.cast_unsigned())
}

/// Splits `x` into `m * 2^e` with `m` in `[3/4, 3/2)`, returning `m` and `e` as a float.
/// The sign of `x` is ignored.
#[inline(always)]
fn split_near_one<T: Float>(x: T) -> (T, T) {
    let mbits = mantissa_bits::<T>();
    let abs_mask = !(ibits::<T>(1) << (mbits + exponent_bits::<T>()));
    let nudge = ibits::<T>(1) << (mbits - ibits::<T>(1));

    let subnormal = x.abs().num_lt(T::MIN_POSITIVE);
    let normalized = subnormal.select(x * subnormal_scale::<T>(), x);

    let bits = normalized.to_bits().cast_signed() & abs_mask;
    let e = ((bits.cast_unsigned() + nudge.cast_unsigned()) >> mbits.cast_unsigned()).cast_signed()
        - exp_bias::<T>();

    let m = scale_exp2(T::from_bits(bits.cast_unsigned()), e);
    let ef = T::from_int(e) - subnormal.select(subnormal_shift::<T>(), T::ZERO);
    (m, ef)
}

// == exp and log ==

/// Computes e^x.
#[inline]
pub(crate) fn exp<T: Float>(x: T) -> T {
    // SLEEF `xexpf`

    // Split x = q * ln(2) + r
    // where q is an integer and r is in [-ln(2)/2, ln(2)/2]
    // such that e^x = 2^q * e^r.
    let q = (x * T::LOG2_E).round();

    // Cody–Waite reduction: subtract q * ln(2) with ln(2) given as two constants
    // so each q * c is exact and no low-order bits of r are lost to cancellation.
    let mut r = q * -k::<T>(LN_2_UPPER) + x;
    r = q * -k::<T>(LN_2_LOWER) + r;

    // Minimax polynomial for e^r on the reduced interval, in Horner form.
    let mut u = k::<T>(0.000198527617612853646278381);
    u = u * r + k::<T>(0.00139304355252534151077271);
    u = u * r + k::<T>(0.00833336077630519866943359);
    u = u * r + k::<T>(0.0416664853692054748535156);
    u = u * r + k::<T>(0.166666671633720397949219);
    u = u * r + k::<T>(0.5);

    // Assemble e^r = 1 + r + r^2 * u.
    u = (r * r) * u + r + T::ONE;

    // Undo the reduction: e^x = 2^q * e^r.
    let mut result = pow2i::<T>(q.to_int()) * u;

    // Above EXP_HI, the result overflows to +inf.
    result = x.num_gt(exp_hi::<T>()).select(T::INFINITY, result);

    // Below EXP_LO, the result flushes to 0.
    result = x.num_lt(exp_lo::<T>()).select(T::ZERO, result);

    // NaN inputs propagate through.
    x.is_nan().select(x, result)
}

/// Computes e^x - 1.
#[inline]
pub(crate) fn expm1<T: Float>(x: T) -> T {
    // SLEEF `expm1fk`

    // Same split and Cody–Waite reduction as `exp`.
    let q = (x * T::LOG2_E).round();

    let mut r = q * -k::<T>(LN_2_UPPER) + x;
    r = q * -k::<T>(LN_2_LOWER) + r;

    // Minimax polynomial for e^r - 1 on the reduced interval, in Horner form.
    let mut u = k::<T>(0.000198527617612853646278381);
    u = u * r + k::<T>(0.00139304355252534151077271);
    u = u * r + k::<T>(0.00833336077630519866943359);
    u = u * r + k::<T>(0.0416664853692054748535156);
    u = u * r + k::<T>(0.166666671633720397949219);
    u = u * r + k::<T>(0.5);

    // Assemble e^r - 1 = r + r^2 * u.
    // The trailing `+ 1` from `exp` is dropped so this stays exact
    // for small r instead of cancelling against a later `- 1`.
    u = (r * r) * u + r;

    // Undo the reduction: e^x - 1 = 2^q * (e^r - 1) + (2^q - 1).
    let two_q = pow2i::<T>(q.to_int());
    let mut result = two_q * u + (two_q - T::ONE);

    // Above EXP_HI, the result overflows to +inf.
    result = x.num_gt(exp_hi::<T>()).select(T::INFINITY, result);

    // Below EXP_LO, e^x flushes to 0, so e^x - 1 = -1.
    result = x.num_lt(exp_lo::<T>()).select(-T::ONE, result);

    // NaN inputs propagate through.
    x.is_nan().select(x, result)
}

/// Computes 2^x.
#[inline]
pub(crate) fn exp2<T: Float>(x: T) -> T {
    exp(x * T::LN_2)
}

/// Computes the natural logarithm ln(x).
#[inline]
pub(crate) fn ln<T: Float>(d: T) -> T {
    // SLEEF `xlogf`

    // Split d = m * 2^e such that ln(d) = ln(m) + e * ln(2).
    // The split puts m in [3/4, 3/2), centered on 1 where the series
    // below converges fastest.
    let (m, ef) = split_near_one(d);

    // ln(m) = 2 * atanh(x) = 2 * (x + x^3/3 + x^5/5 + ...)
    // where x = (m - 1)/(m + 1), evaluated as 2 * x + x^3 * P(x^2)
    // where P is a minimax polynomial in Horner form.
    let x = (m - T::ONE) / (m + T::ONE);
    let x2 = x * x;
    let mut t = k::<T>(0.2392828464508056640625);
    t = t * x2 + k::<T>(0.28518211841583251953125);
    t = t * x2 + k::<T>(0.400005877017974853515625);
    t = t * x2 + k::<T>(0.666666686534881591796875);

    // ln(d) = ln(m) + e * ln(2).
    let mut result = (x2 * x) * t + x * k::<T>(2.0) + ef * T::LN_2;

    // d < 0 -> NaN
    result = d.num_lt(T::ZERO).select(T::NAN, result);

    // d == 0 -> -inf
    result = d.num_eq(T::ZERO).select(T::NEG_INFINITY, result);

    // d == +inf -> +inf
    result = d.num_eq(T::INFINITY).select(T::INFINITY, result);

    // NaN inputs propagate through.
    d.is_nan().select(d, result)
}

/// Computes log2(x).
#[inline]
pub(crate) fn log2<T: Float>(x: T) -> T {
    ln(x) * T::LOG2_E
}

/// Computes log10(x).
#[inline]
pub(crate) fn log10<T: Float>(x: T) -> T {
    ln(x) * T::LOG10_E
}

/// Computes log_base(x).
#[inline]
pub(crate) fn log<T: Float>(x: T, base: T) -> T {
    ln(x) / ln(base)
}

// == trig ==

/// Computes sin(x) and cos(x).
#[inline]
pub(crate) fn sin_cos<T: Float>(d: T) -> (T, T) {
    // SLEEF `xsincosf`, except the sin polynomial is from `xsinf`,
    // since it's more accurate than what `xsincosf` uses.

    // Split d = q * (pi / 2) + r
    // where q is an integer and r is in [-pi/4, pi/4]
    // then evaluate the polynomials on r and recover the full-range result from q.
    let q = (d * T::FRAC_2_PI).round();

    // Cody-Waite reduction: subtract q*(pi/2) with pi/2 given as four constants
    // so each q * c is exact and no low-order bits of r are lost to cancellation.
    let mut r = q * -k::<T>(1.5703125) + d;
    r = q * -k::<T>(0.00048351287841796875) + r;
    r = q * -k::<T>(3.13855707645416259765e-07) + r;
    r = q * -k::<T>(6.0771006282767103810e-11) + r;

    let s = r * r;

    // Minimax polynomial for sin(r) on [-pi/4, pi/4] (Horner in s = r^2).
    let mut su = k::<T>(2.6083159809786593541503e-06);
    su = su * s + k::<T>(-0.0001981069071916863322258);
    su = su * s + k::<T>(0.00833307858556509017944336);
    su = su * s + k::<T>(-0.166666597127914428710938);
    let sin_r = (r * s) * su + r;

    // Minimax polynomial for cos(r) on [-pi/4, pi/4] (Horner in s = r^2).
    let mut cu = k::<T>(-2.71811842367242206819355e-07);
    cu = cu * s + k::<T>(2.47990446951007470488548e-05);
    cu = cu * s + k::<T>(-0.00138888787478208541870117);
    cu = cu * s + k::<T>(0.0416666641831398010253906);
    cu = cu * s + k::<T>(-0.5);
    let cos_r = cu * s + T::ONE;

    // Map q mod 4 to the correct quadrant.
    let q4 = q - (q * k::<T>(0.25)).floor() * k::<T>(4.0);

    // Odd quadrants swap sin and cos.
    let swap = q4.num_eq(T::ONE) | q4.num_eq(k::<T>(3.0));
    let mut sin = swap.select(cos_r, sin_r);
    let mut cos = swap.select(sin_r, cos_r);

    // Negate sin in quadrants 2 and 3, and cos in quadrants 1 and 2.
    sin = q4.num_ge(k::<T>(2.0)).select(-sin, sin);
    cos = (q4.num_eq(T::ONE) | q4.num_eq(k::<T>(2.0))).select(-cos, cos);

    (sin, cos)
}

/// Computes sin(x).
#[inline]
pub(crate) fn sin<T: Float>(d: T) -> T {
    // SLEEF `xsinf`.

    // Split d = q * pi + r
    // where q is an integer and r is in [-pi/2, pi/2]
    // so sin(d) = (-1)^q * sin(r).
    let q = (d * T::FRAC_1_PI).round();

    // Cody-Waite reduction: subtract q*pi with pi given as four constants
    // so each q * c is exact and no low-order bits of r are lost to cancellation.
    let mut r = q * -k::<T>(3.140625) + d;
    r = q * -k::<T>(0.0009670257568359375) + r;
    r = q * -k::<T>(6.2771141529083251953e-07) + r;
    r = q * -k::<T>(1.2154201256553420762e-10) + r;

    let s = r * r;

    // Minimax polynomial for sin(r) on [-pi/2, pi/2] (Horner in s = r^2).
    let mut u = k::<T>(2.6083159809786593541503e-06);
    u = u * s + k::<T>(-0.0001981069071916863322258);
    u = u * s + k::<T>(0.00833307858556509017944336);
    u = u * s + k::<T>(-0.166666597127914428710938);
    let sin_r = (r * s) * u + r;

    // Odd q negates the result, since sin(r + q*pi) = (-1)^q * sin(r).
    // The low bit of the integer q selects the negation.
    negate_if_odd(sin_r, q)
}

/// Computes cos(x).
#[inline]
pub(crate) fn cos<T: Float>(d: T) -> T {
    // SLEEF `xcosf`

    // Split d = q * (pi/2) + r
    // where q = 2n + 1 is odd and r is in [-pi/2, pi/2].
    //
    // An odd q turns cos into a sin evaluation: cos(d) = +/- sin(r).
    let n = (d * T::FRAC_1_PI - T::HALF).round();
    let q = k::<T>(2.0) * n + T::ONE;

    // Cody-Waite reduction: subtract q*(pi/2) with pi/2 given as four constants.
    let mut r = q * -k::<T>(1.5703125) + d;
    r = q * -k::<T>(0.00048351287841796875) + r;
    r = q * -k::<T>(3.13855707645416259765e-07) + r;
    r = q * -k::<T>(6.0771006282767103810e-11) + r;

    let s = r * r;

    // Minimax polynomial for sin(r) on [-pi/2, pi/2] (Horner in s = r^2).
    let mut u = k::<T>(2.6083159809786593541503e-06);
    u = u * s + k::<T>(-0.0001981069071916863322258);
    u = u * s + k::<T>(0.00833307858556509017944336);
    u = u * s + k::<T>(-0.166666597127914428710938);
    let sin_r = (r * s) * u + r;

    // q mod 4 == 1 (n even) negates the result. n + 1 is odd exactly when n is even,
    // so negating on its parity applies the sign.
    negate_if_odd(sin_r, n + T::ONE)
}

/// Computes tan(x).
#[inline]
pub(crate) fn tan<T: Float>(d: T) -> T {
    // SLEEF `xtanf`.

    // Split d = q * (pi/2) + r
    // where q is an integer and r is in [-pi/4, pi/4].
    let q = (d * T::FRAC_2_PI).round();

    // Cody-Waite reduction: subtract q*(pi/2) with pi/2 given as four constants.
    let mut r = q * -k::<T>(1.5703125) + d;
    r = q * -k::<T>(0.00048351287841796875) + r;
    r = q * -k::<T>(3.13855707645416259765e-07) + r;
    r = q * -k::<T>(6.0771006282767103810e-11) + r;

    let s = r * r;

    // Minimax polynomial for tan(r) on [-pi/4, pi/4] (Horner in s = r^2).
    let mut u = k::<T>(0.00927245803177356719970703);
    u = u * s + k::<T>(0.00331984995864331722259521);
    u = u * s + k::<T>(0.0242998078465461730957031);
    u = u * s + k::<T>(0.0534495301544666290283203);
    u = u * s + k::<T>(0.133383005857467651367188);
    u = u * s + k::<T>(0.333331853151321411132812);
    let tan_r = (r * s) * u + r;

    // Odd quadrants map tan to its negated reciprocal: tan(r + pi/2) = -1/tan(r).
    let q_odd = (q * T::HALF).fract().num_ne(T::ZERO);
    let result = q_odd.select(-(T::ONE / tan_r), tan_r);

    // Canonicalize infinities, while preserving NaN inputs and signed zero.
    let result = (!d.is_finite()).select(T::NAN, result);
    (d.is_nan() | d.num_eq(T::ZERO)).select(d, result)
}

/// Computes atan(x).
#[inline]
pub(crate) fn atan<T: Float>(x: T) -> T {
    // Adapted from cephes `atanf`

    // Work on |x|, restoring the sign at the end.
    let neg = x.num_lt(T::ZERO);
    let ax = x.abs();

    // For |x| > 1, use atan(x) = pi/2 - atan(1/x) so `atan_unit` always sees [0, 1].
    let inv = ax.num_gt(T::ONE);
    let z = inv.select(T::ONE / ax, ax);

    // Compute atan(z) in [0, 1].
    let r = atan_unit(z);
    let r = inv.select(T::FRAC_PI_2 - r, r);

    // Restore the sign of the original input.
    neg.select(-r, r)
}

/// Computes atan of a value in `[0, 1]`.
#[inline(always)]
fn atan_unit<T: Float>(x: T) -> T {
    // Adapted from cephes `atanf`

    // Above tan(pi/8), fold x into [0, tan(pi/8)] and add pi/4 back at the end.
    let big = x.num_gt(k::<T>(0.4142135623730950488));
    let z = big.select((x - T::ONE) / (x + T::ONE), x);

    // Minimax polynomial for atan(z) = z + z^3 * P(z^2) on [0, tan(pi/8)] (Horner in z^2).
    let z2 = z * z;
    let mut p = k::<T>(8.05374449538e-2);
    p = p * z2 + k::<T>(-1.38776856032e-1);
    p = p * z2 + k::<T>(1.99777106478e-1);
    p = p * z2 + k::<T>(-3.33329491539e-1);
    let r = (z * z2) * p + z;

    // Add pi/4 back for the folded case.
    big.select(r + T::FRAC_PI_4, r)
}

/// Computes atan2(y, x).
#[inline]
pub(crate) fn atan2<T: Float>(y: T, x: T) -> T {
    // Adapted from cephes `atan2f`

    // Compute atan of the ratio |min(y, x)| / |max(y, x)|,
    // then reflect into the correct quadrant.
    let ax = x.abs();
    let ay = y.abs();
    let min = NumOrd::min(ax, ay);
    let max = NumOrd::max(ax, ay);
    let a = min / max;

    let mut r = atan_unit(a);

    // |y| > |x| -> the ratio was inverted, so reflect the angle
    r = ay.num_gt(ax).select(T::FRAC_PI_2 - r, r);

    // x < 0 -> angle is measured from pi
    r = x.num_lt(T::ZERO).select(T::PI - r, r);

    // atan2 is odd in y -> carry y's sign
    r = r.copysign(y);

    // (0, 0) yields 0 to match `f32::atan2` and avoid a NaN from 0/0.
    (x.num_eq(T::ZERO) & y.num_eq(T::ZERO)).select(T::ZERO, r)
}

/// Evaluates the degree-7 minimax polynomial `P(x)` for which `acos(x) = sqrt(1 - x) * P(x)`
/// on `[0, 1]` (DirectXMath `XMScalarAcos`). Shared by [`acos`] and [`asin`].
#[inline(always)]
fn acos_poly<T: Float>(x: T) -> T {
    let mut u = k::<T>(-0.0012624911);
    u = u * x + k::<T>(0.0066700901);
    u = u * x + k::<T>(-0.0170881256);
    u = u * x + k::<T>(0.0308918810);
    u = u * x + k::<T>(-0.0501743046);
    u = u * x + k::<T>(0.0889789874);
    u = u * x + k::<T>(-0.2145988016);
    u * x + k::<T>(1.5707963050)
}

/// Computes asin(x).
#[inline]
pub(crate) fn asin<T: Float>(d: T) -> T {
    // asin(x) = pi/2 - acos(|x|) for x in [-1, 1] (DirectXMath `XMScalarASin`)
    let x = d.abs();
    let acos_abs = acos_poly(x) * (T::ONE - x).sqrt();
    (T::FRAC_PI_2 - acos_abs).copysign(d)
}

/// Computes acos(x).
#[inline]
pub(crate) fn acos<T: Float>(d: T) -> T {
    // acos(x) = sqrt(1 - x) * P(x) for x in [0, 1] (DirectXMath `XMScalarAcos`)
    let x = d.abs();
    let r = acos_poly(x) * (T::ONE - x).sqrt();

    // Reflect across pi for x < 0.
    d.num_lt(T::ZERO).select(T::PI - r, r)
}

// == hyperbolic ==

/// Computes sinh(x) from the definition `sinh(x) = (e^x - e^-x) / 2`.
#[inline]
pub(crate) fn sinh<T: Float>(x: T) -> T {
    // Written with expm1 rather than exp to avoid (e^x - e^-x) cancellation near zero
    (expm1(x) - expm1(-x)) * T::HALF
}

/// Computes cosh(x) from the definition `cosh(x) = (e^x + e^-x) / 2`.
#[inline]
pub(crate) fn cosh<T: Float>(x: T) -> T {
    let e = exp(x);
    (e + T::ONE / e) * T::HALF
}

/// Computes tanh(x) via `tanh(x) = expm1(2|x|) / (expm1(2|x|) + 2)`.
#[inline]
pub(crate) fn tanh<T: Float>(x: T) -> T {
    // Work on 2|x|, restoring the sign at the end.
    let a2 = x.abs() + x.abs();

    let u = expm1(a2);
    let t = u / (u + k::<T>(2.0));

    // Clamp to 1 once e^(2|x|) overflows.
    let t = a2.num_gt(exp_hi::<T>()).select(T::ONE, t);

    // Restore the sign of the original input.
    t.copysign(x)
}

/// Computes asinh(x).
#[inline]
pub(crate) fn asinh<T: Float>(x: T) -> T {
    let a = x.abs();

    let large = a.num_gt(T::MAX.sqrt());
    let arg = large.select(a, a + (a * a + T::ONE).sqrt());
    let result = ln(arg) + large.select(T::LN_2, T::ZERO);

    let result = a.num_lt(T::EPSILON.sqrt()).select(a, result);
    let result = result.copysign(x);

    x.is_nan().select(x, result)
}

/// Computes acosh(x).
#[inline]
pub(crate) fn acosh<T: Float>(x: T) -> T {
    let large = x.num_gt(T::MAX.sqrt());
    let arg = large.select(x, x + ((x - T::ONE) * (x + T::ONE)).sqrt());
    ln(arg) + large.select(T::LN_2, T::ZERO)
}

/// Computes atanh(x).
#[inline]
pub(crate) fn atanh<T: Float>(x: T) -> T {
    let a = x.abs();
    let result = ln((T::ONE + a) / (T::ONE - a)) * T::HALF;
    let result = a.num_lt(T::EPSILON.sqrt()).select(a, result);
    let result = result.copysign(x);
    x.is_nan().select(x, result)
}

// == roots and powers ==

/// Computes the cube root of `x`.
#[inline]
pub(crate) fn cbrt<T: Float>(x: T) -> T {
    // SLEEF `cbrtf`

    // Work on the absolute value, restoring the sign at the end.
    let a = x.abs();

    let subnormal = a.num_lt(T::MIN_POSITIVE);
    let a = subnormal.select(a * subnormal_scale::<T>(), a);
    let unscale = subnormal.select(subnormal_cbrt_unscale::<T>(), T::ONE);
    let three = ibits::<T>(3);

    // a = m * 2^e
    // where e = floor(log2(a)) + 1 and m is in [0.5, 1)
    let e = ilogb(a) + ibits::<T>(1);
    let m = scale_exp2(a, e);

    // Split e into 3 * scale + rem, where rem is in {0, 1, 2}.
    // Biasing by 6144, a multiple of 3, keeps the operands non-negative
    // so the division and remainder behave regardless of the sign of e.
    let biased = e + ibits::<T>(6144);
    let rem = T::from_int(biased % three);
    let scale = biased / three - ibits::<T>(2048);

    // cbrt(a) = cbrt(m) * 2^scale * 2^(rem/3), undoing any subnormal scaling.
    let mut factor = pow2i::<T>(scale) * unscale;
    factor = rem
        .num_eq(T::ONE)
        .select(factor * k::<T>(1.2599210498948731647672106), factor);
    factor = rem
        .num_eq(k::<T>(2.0))
        .select(factor * k::<T>(1.5874010519681994747517056), factor);

    // Minimax polynomial approximating m^(-1/3) on [0.5, 1) (Horner in m)
    let mut p = k::<T>(-0.601564466953277587890625);
    p = p * m + k::<T>(2.8208892345428466796875);
    p = p * m + k::<T>(-5.532182216644287109375);
    p = p * m + k::<T>(5.898262500762939453125);
    p = p * m + k::<T>(-3.8095417022705078125);
    p = p * m + k::<T>(2.2241256237030029296875);

    // cbrt(m) = m * p^2, refined by one Newton step, then rescaled.
    let mut y = m * p * p;
    y = (y - k::<T>(2.0 / 3.0) * y * (y * p - T::ONE)) * factor;
    y = y.copysign(x);

    // Preserve zero, infinity, and NaN inputs.
    y = a.num_eq(T::ZERO).select(x, y);
    y = a.num_eq(T::INFINITY).select(x, y);
    x.is_nan().select(x, y)
}

/// Computes sqrt(x^2 + y^2) without overflowing or underflowing.
#[inline]
pub(crate) fn hypot<T: Float>(x: T, y: T) -> T {
    let ax = x.abs();
    let ay = y.abs();
    let swap = ax.num_lt(ay);
    let hi = swap.select(ay, ax);
    let lo = swap.select(ax, ay);
    let ratio = lo / hi;
    let result = hi * (T::ONE + ratio * ratio).sqrt();

    let result = hi.num_eq(T::ZERO).select(T::ZERO, result);
    let result = (x.is_nan() | y.is_nan()).select(T::NAN, result);
    (x.is_infinite() | y.is_infinite()).select(T::INFINITY, result)
}

/// Computes x^y.
#[inline]
pub(crate) fn powf<T: Float>(x: T, y: T) -> T {
    // `x^y = exp(y * ln|x|)`
    let mut r = exp(y * ln(x.abs()));

    // Handle special cases to match `f32::powf` and `f64::powf`.

    // For negative bases, an integer exponent keeps a real result,
    // while a non-integer exponent results in NaN.
    let yint = y.round();
    let y_is_int = yint.num_eq(y);
    let y_is_odd = (yint * T::HALF).fract().num_ne(T::ZERO);
    let xneg = x.num_lt(T::ZERO);
    r = (xneg & !y_is_int).select(T::NAN, r);
    r = (xneg & y_is_odd).select(-r, r);

    // `x^0 == 1` and `1^y == 1` for every `y`, including NaN.
    r = x.num_eq(T::ONE).select(T::ONE, r);
    y.num_eq(T::ZERO).select(T::ONE, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sweeps a unary approximation against the `std` reference over `[lo, hi]`.
    fn sweep_unary(
        name: &str,
        lo: f32,
        hi: f32,
        tol: f32,
        approx: fn(f32) -> f32,
        reference: fn(f32) -> f32,
    ) {
        let mut worst = 0.0f32;
        let mut at = 0.0f32;
        let n = 4000;
        for i in 0..=n {
            let x = lo + (hi - lo) * ((i as f32) / (n as f32));
            let e = reference(x);
            let a = approx(x);
            assert!(
                a.is_nan() == e.is_nan() && a.is_infinite() == e.is_infinite(),
                "{name}: got {a:e}, wanted {e:e} at x={x:e}"
            );
            let err = if e.abs() > 1.0 {
                ((a - e) / e).abs()
            } else {
                (a - e).abs()
            };
            if err > worst {
                worst = err;
                at = x;
            }
        }
        assert!(
            worst <= tol,
            "{name}: worst rel/abs err {worst:e} at x={at} exceeds tol {tol:e}"
        );
    }

    #[test]
    fn accuracy_exp_log() {
        sweep_unary("exp", -20.0, 20.0, 3e-6, exp, f32::exp);
        sweep_unary("exp2", -20.0, 20.0, 3e-6, exp2, f32::exp2);
        sweep_unary("ln", 1e-6, 1e6, 3e-6, ln, f32::ln);
        sweep_unary("ln-big", 1e30, f32::MAX, 3e-6, ln, f32::ln);
        sweep_unary("ln-small", f32::MIN_POSITIVE, 1e-30, 3e-6, ln, f32::ln);
        sweep_unary(
            "ln-subnormal",
            f32::from_bits(1),
            f32::MIN_POSITIVE,
            3e-6,
            ln,
            f32::ln,
        );
        sweep_unary(
            "cbrt-subnormal",
            f32::from_bits(1),
            f32::MIN_POSITIVE,
            3e-6,
            cbrt,
            f32::cbrt,
        );
        sweep_unary("log2", 1e-6, 1e6, 3e-6, log2, f32::log2);
        sweep_unary("log10", 1e-6, 1e6, 3e-6, log10, f32::log10);
        sweep_unary("cbrt", -1000.0, 1000.0, 3e-6, cbrt, f32::cbrt);
        sweep_unary("cbrt-big", 1e30, 3e38, 3e-6, cbrt, f32::cbrt);
        sweep_unary("cbrt-small", 1e-30, 1e-20, 3e-6, cbrt, f32::cbrt);
    }

    #[test]
    fn accuracy_trig() {
        sweep_unary("sin", -12.0, 12.0, 3e-7, sin, f32::sin);
        sweep_unary("cos", -12.0, 12.0, 3e-7, cos, f32::cos);
        // The Cody-Waite reduction stays accurate until argument cancellation takes over
        // around |x| ~= 1e5. Past this, a Payne-Hanek reduction would be needed.
        sweep_unary("sin-mid", -1e4, 1e4, 3e-6, sin, f32::sin);
        sweep_unary("cos-mid", -1e4, 1e4, 3e-6, cos, f32::cos);
        // Avoid the poles at +/- pi/2, where tan diverges.
        sweep_unary("tan", -1.4, 1.4, 3e-6, tan, f32::tan);
        sweep_unary("atan", -20.0, 20.0, 3e-5, atan, f32::atan);
        sweep_unary("asin", -1.0, 1.0, 5e-7, asin, f32::asin);
        sweep_unary("acos", -1.0, 1.0, 1e-6, acos, f32::acos);
    }

    #[test]
    fn accuracy_hyperbolic() {
        sweep_unary("sinh", -10.0, 10.0, 3e-6, sinh, f32::sinh);
        sweep_unary("cosh", -10.0, 10.0, 3e-6, cosh, f32::cosh);
        sweep_unary("tanh", -10.0, 10.0, 3e-6, tanh, f32::tanh);
        sweep_unary("asinh", -10.0, 10.0, 3e-5, asinh, f32::asinh);
        sweep_unary("acosh", 1.0, 10.0, 3e-5, acosh, f32::acosh);
        sweep_unary("atanh", -0.99, 0.99, 3e-5, atanh, f32::atanh);
        sweep_unary("asinh-cross", 1e15, 1e25, 3e-5, asinh, f32::asinh);
        sweep_unary("acosh-cross", 1e15, 1e25, 3e-5, acosh, f32::acosh);
        sweep_unary("asinh-big", 1e30, f32::MAX, 3e-5, asinh, f32::asinh);
        sweep_unary("acosh-big", 1e30, f32::MAX, 3e-5, acosh, f32::acosh);
    }

    #[test]
    fn accuracy_binary() {
        // atan2 over a grid.
        let mut worst = 0.0f32;
        for i in -20..=20 {
            for j in -20..=20 {
                let (y, x) = (i as f32 * 0.5, j as f32 * 0.5);
                if x == 0.0 && y == 0.0 {
                    continue;
                }
                let e = y.atan2(x);
                let a = atan2(y, x);
                worst = worst.max((a - e).abs());
            }
        }
        assert!(worst <= 3e-5, "atan2 worst {worst:e}");

        // powf on positive bases.
        let mut wp = 0.0f32;
        for bi in 1..=40 {
            for ei in -20..=20 {
                let (b, e) = (bi as f32 * 0.25, ei as f32 * 0.25);
                let exact = b.powf(e);
                let a = powf(b, e);
                let err = if exact.abs() > 1.0 {
                    ((a - exact) / exact).abs()
                } else {
                    (a - exact).abs()
                };
                wp = wp.max(err);
            }
        }
        assert!(wp <= 2e-6, "powf worst {wp:e}");
    }

    #[test]
    fn edge_cases() {
        assert_eq!(exp(0.0f32), 1.0);
        assert_eq!(ln(1.0f32), 0.0);
        assert_eq!(ln(0.0f32), f32::NEG_INFINITY);
        assert!(ln(-1.0f32).is_nan());
        assert_eq!(hypot(3.0f32, 4.0), 5.0);
        assert_eq!(powf(1.0f32, f32::NAN), 1.0);
        assert_eq!(powf(f32::NAN, 0.0), 1.0);
        assert_eq!(powf(0.0f32, 0.0), 1.0);
        assert_eq!(cbrt(0.0f32), 0.0);
        assert_eq!(cbrt(-0.0f32).to_bits(), (-0.0f32).to_bits());
        assert_eq!(cbrt(8.0f32), 2.0);
        assert_eq!(cbrt(-8.0f32), -2.0);
        assert_eq!(cbrt(f32::INFINITY), f32::INFINITY);
        assert_eq!(cbrt(f32::NEG_INFINITY), f32::NEG_INFINITY);
        assert!(cbrt(f32::NAN).is_nan());

        // tanh saturates to +/-1 instead of producing NaN for large |x|.
        assert_eq!(tanh(100.0f32), 1.0);
        assert_eq!(tanh(-100.0f32), -1.0);
        assert_eq!(tanh(0.0f32), 0.0);

        // expm1 keeps relative accuracy near zero where `exp(x) - 1` cannot.
        for i in -50..=50 {
            let x = i as f32 * 1e-3;
            let e = x.exp_m1();
            let a = expm1(x);
            let err = if e.abs() > 0.0 {
                ((a - e) / e).abs()
            } else {
                a.abs()
            };
            assert!(err <= 4e-6, "expm1 rel err {err:e} at x={x}");
        }
    }

    /// A NaN with a distinctive payload.
    const PAYLOAD_NAN: f32 = f32::from_bits(0x7fc0_0005);

    /// Sweeps through every `f32` subnormal and checks that the `ln` and `cbrt`
    /// approximations are accurate.
    #[test]
    fn subnormals_are_normalized() {
        let (mut worst_ln, mut worst_cbrt) = (0.0f32, 0.0f32);
        let (mut at_ln, mut at_cbrt) = (0.0f32, 0.0f32);
        for bits in 1..(1u32 << 23) {
            let x = f32::from_bits(bits);
            let e = x.ln();
            let err = ((ln(x) - e) / e).abs();
            if err > worst_ln {
                worst_ln = err;
                at_ln = x;
            }
            let e = x.cbrt();
            let err = ((cbrt(x) - e) / e).abs();
            if err > worst_cbrt {
                worst_cbrt = err;
                at_cbrt = x;
            }
        }
        assert!(worst_ln <= 3e-6, "ln worst {worst_ln:e} at {at_ln:e}");
        assert!(
            worst_cbrt <= 3e-6,
            "cbrt worst {worst_cbrt:e} at {at_cbrt:e}"
        );
    }

    #[test]
    fn tan_edge_cases() {
        assert_eq!(tan(f32::INFINITY).to_bits(), f32::NAN.to_bits());
        assert_eq!(tan(f32::NEG_INFINITY).to_bits(), f32::NAN.to_bits());
        assert_eq!(tan(PAYLOAD_NAN).to_bits(), PAYLOAD_NAN.to_bits());
        assert_eq!(tan(-f32::NAN).to_bits(), (-f32::NAN).to_bits());
        assert_eq!(tan(0.0f32).to_bits(), 0.0f32.to_bits());
        assert_eq!(tan(-0.0f32).to_bits(), (-0.0f32).to_bits());
    }

    #[test]
    fn inverse_hyperbolic_edge_cases() {
        let tiny = 1e-30f32;

        // Zeros, tiny inputs, and NaN values return the input.
        for f in [asinh, atanh] {
            assert_eq!(f(0.0).to_bits(), 0.0f32.to_bits());
            assert_eq!(f(-0.0).to_bits(), (-0.0f32).to_bits());
            assert_eq!(f(tiny).to_bits(), tiny.to_bits());
            assert_eq!(f(-tiny).to_bits(), (-tiny).to_bits());
            assert_eq!(f(PAYLOAD_NAN).to_bits(), PAYLOAD_NAN.to_bits());
        }

        // Large inputs must stay finite rather than collapsing to infinity.
        for x in [1e30f32, 1e35, 1e38, f32::MAX] {
            let (a, e) = (asinh(x), x.asinh());
            assert!(a.is_finite(), "asinh({x:e}) = {a:e}");
            assert!(
                ((a - e) / e).abs() <= 3e-5,
                "asinh({x:e}) = {a:e}, wanted {e:e}"
            );

            assert_eq!(asinh(-x).to_bits(), (-asinh(x)).to_bits());

            let (a, e) = (acosh(x), x.acosh());
            assert!(a.is_finite(), "acosh({x:e}) = {a:e}");
            assert!(
                ((a - e) / e).abs() <= 3e-5,
                "acosh({x:e}) = {a:e}, wanted {e:e}"
            );
        }

        assert_eq!(asinh(f32::INFINITY), f32::INFINITY);
        assert_eq!(asinh(f32::NEG_INFINITY), f32::NEG_INFINITY);

        assert_eq!(acosh(1.0f32), 0.0);
        assert_eq!(acosh(f32::INFINITY), f32::INFINITY);

        // Below the domain.
        assert!(acosh(0.5f32).is_nan());
        assert!(acosh(f32::NAN).is_nan());

        assert_eq!(atanh(1.0f32), f32::INFINITY);
        assert_eq!(atanh(-1.0f32), f32::NEG_INFINITY);

        // Outside the domain.
        assert!(atanh(2.0f32).is_nan());
        assert!(atanh(-2.0f32).is_nan());
    }

    #[test]
    fn hypot_edge_cases() {
        assert_eq!(hypot(3.0f32, 4.0), 5.0);
        assert_eq!(hypot(-3.0f32, -4.0), 5.0);
        assert_eq!(hypot(0.0f32, 0.0), 0.0);
        assert_eq!(hypot(-0.0f32, -0.0).to_bits(), 0.0f32.to_bits());

        // Squaring either side overflows or underflows way earlier than the result does,
        // so these must not collapse to infinity or zero.
        for (x, y) in [(1e30f32, 1e30f32), (1e-25, 1e-25), (f32::MAX, 1.0)] {
            for (x, y) in [(x, y), (y, x)] {
                let (a, e) = (hypot(x, y), x.hypot(y));
                assert!(a.is_finite() && a > 0.0, "hypot({x:e}, {y:e}) = {a:e}");
                assert!(
                    ((a - e) / e).abs() <= 3e-6,
                    "hypot({x:e}, {y:e}) = {a:e}, wanted {e:e}"
                );
            }
        }

        let min = f32::MIN_POSITIVE;
        assert!(hypot(min, min) > min);

        // A genuine overflow should still overflow.
        assert_eq!(hypot(f32::MAX, f32::MAX), f32::INFINITY);

        // IEEE-754 requires hypot to choose infinity over NaN.
        assert_eq!(hypot(f32::INFINITY, f32::NAN), f32::INFINITY);
        assert_eq!(hypot(f32::NAN, f32::NEG_INFINITY), f32::INFINITY);
        assert_eq!(hypot(f32::NEG_INFINITY, 1.0), f32::INFINITY);
        assert!(hypot(f32::NAN, 1.0).is_nan());
        assert!(hypot(1.0, f32::NAN).is_nan());
    }
}

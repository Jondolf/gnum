use crate::{
    simd::SimdPartialOrd,
    traits::{Num, Signed},
};

/// Basic mathematical constants for [`Real`] number types such as [`f32`] and [`f64`].
pub trait RealConstants {
    /// Euler's number (e)
    const E: Self;
    /// 1/π
    const FRAC_1_PI: Self;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;
    /// 2/π
    const FRAC_2_PI: Self;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;
    /// ln(2)
    const LN_2: Self;
    /// ln(10)
    const LN_10: Self;
    /// log<sub>2</sub>(10)
    const LOG2_10: Self;
    /// log<sub>2</sub>(e)
    const LOG2_E: Self;
    /// log<sub>10</sub>(2)
    const LOG10_2: Self;
    /// log<sub>10</sub>(e)
    const LOG10_E: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// sqrt(2)
    const SQRT_2: Self;
    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: Self;
    /// The Euler-Mascheroni constant (γ)
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self;
    /// 1/sqrt(2π)
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self;
    /// 1/sqrt(π)
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self;
    /// The golden ratio (φ)
    #[cfg(feature = "more_float_constants")]
    const PHI: Self;
    /// sqrt(3)
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self;
}

macro_rules! impl_real_constants {
    ($($real:tt),*) => {
        $(
            impl RealConstants for $real {
                const E: Self = core::$real::consts::E;
                const FRAC_1_PI: Self = core::$real::consts::FRAC_1_PI;
                const FRAC_1_SQRT_2: Self = core::$real::consts::FRAC_1_SQRT_2;
                const FRAC_2_PI: Self = core::$real::consts::FRAC_2_PI;
                const FRAC_2_SQRT_PI: Self = core::$real::consts::FRAC_2_SQRT_PI;
                const FRAC_PI_2: Self = core::$real::consts::FRAC_PI_2;
                const FRAC_PI_3: Self = core::$real::consts::FRAC_PI_3;
                const FRAC_PI_4: Self = core::$real::consts::FRAC_PI_4;
                const FRAC_PI_6: Self = core::$real::consts::FRAC_PI_6;
                const FRAC_PI_8: Self = core::$real::consts::FRAC_PI_8;
                const LN_2: Self = core::$real::consts::LN_2;
                const LN_10: Self = core::$real::consts::LN_10;
                const LOG2_10: Self = core::$real::consts::LOG2_10;
                const LOG2_E: Self = core::$real::consts::LOG2_E;
                const LOG10_2: Self = core::$real::consts::LOG10_2;
                const LOG10_E: Self = core::$real::consts::LOG10_E;
                const PI: Self = core::$real::consts::PI;
                const SQRT_2: Self = core::$real::consts::SQRT_2;
                const TAU: Self = core::$real::consts::TAU;
                #[cfg(feature = "more_float_constants")]
                const EGAMMA: Self = core::$real::consts::EGAMMA;
                #[cfg(feature = "more_float_constants")]
                const FRAC_1_SQRT_3: Self = core::$real::consts::FRAC_1_SQRT_3;
                #[cfg(feature = "more_float_constants")]
                const FRAC_1_SQRT_2PI: Self = core::$real::consts::FRAC_1_SQRT_2PI;
                #[cfg(feature = "more_float_constants")]
                const FRAC_1_SQRT_PI: Self = core::$real::consts::FRAC_1_SQRT_PI;
                #[cfg(feature = "more_float_constants")]
                const PHI: Self = core::$real::consts::PHI;
                #[cfg(feature = "more_float_constants")]
                const SQRT_3: Self = core::$real::consts::SQRT_3;
            }
        )*
    };
}

impl_real_constants!(f32, f64);

/// A trait for [real number] types such as [`f32`] and [`f64`].
///
/// [real numbers]: https://en.wikipedia.org/wiki/Real_number
pub trait Real: Num + Signed + RealConstants + SimdPartialOrd {
    /// Returns the largest integer less than or equal to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(x.floor(), 3.0);
    /// assert_eq!(y.floor(), -4.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(x.ceil(), 4.0);
    /// assert_eq!(y.ceil(), -3.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "ceiling")]
    fn ceil(self) -> Self;

    /// Returns the integer nearest to `self`. If a value is half-way
    /// between two integers, rounds away from zero.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.7;
    /// let y: f32 = 3.5;
    /// let z: f32 = -3.5;
    ///
    /// assert_eq!(x.round(), 4.0);
    /// assert_eq!(y.round(), 4.0);
    /// assert_eq!(z.round(), -4.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn round(self) -> Self;

    /// Returns the integer part of `self`. This means that noh-integer numbers
    /// are always truncated towards zero.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(x.trunc(), 3.0);
    /// assert_eq!(y.trunc(), -3.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "truncate")]
    fn trunc(self) -> Self;

    /// Returns the fractional part of `self`.
    ///
    /// This is semantically equivalent to `self - self.trunc()`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(x.fract(), 0.7);
    /// assert_eq!(y.fract(), -0.7);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn fract(self) -> Self;

    /// Returns the Euclidean division of `self` by `rhs`.
    ///
    /// This computes the integer `n` such that `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n` such that `self >= n * rhs`.
    ///
    /// # Example
    ///
    /// ```
    /// let a = 7.0;
    /// let b = 4.0;
    ///
    /// assert_eq!(a.div_euclid(b), 1.0); // 7 >= 4 * 1.0
    /// assert_eq!(a.div_euclid(-b), -1.0); // 7 >= -4 * -1.0
    /// assert_eq!((-a).div_euclid(b), -2.0); // -7 >= 4 * -2.0
    /// assert_eq!((-a).div_euclid(-b), 2.0); // -7 >= -4 * 2.0
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn div_euclid(self, rhs: Self) -> Self;

    /// Returns the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfied `0.0 <= r < abs(rhs)`.
    ///
    /// # Example
    ///
    /// ```
    /// let a = 7.0;
    /// let b = 4.0;
    ///
    /// assert_eq!(a.rem_euclid(b), 3.0);
    /// assert_eq!((-a).rem_euclid(b), 1.0);
    /// assert_eq!(a.rem_euclid(-b), 3.0);
    /// assert_eq!((-a).rem_euclid(-b), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "modulo", alias = "mod")]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Returns the square root of `self`.
    ///
    /// `self` must be non-negative.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 4.0;
    /// let y: f32 = 2.0;
    ///
    /// assert_eq!(x.sqrt(), 2.0);
    /// assert_eq!(y.sqrt(), 1.4142135);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "squareRoot")]
    fn sqrt(self) -> Self;

    /// Returns `pow(e, self)`, (the exponential function).
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.exp(), f32::E);
    /// assert_eq!(y.exp(), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp(self) -> Self;

    /// Returns `pow(2, self)`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 2.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.exp2(), 4.0);
    /// assert_eq!(y.exp2(), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp2(self) -> Self;

    /// Returns the logarithm of `self` with respect to a given `base`.
    ///
    /// `self` must be positive.
    ///
    /// # Example
    ///
    /// ```
    /// let base: f32 = 2.0;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.log(base), 4.0);
    /// assert_eq!(y.log(base), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log(self, base: Self) -> Self;

    /// Returns the natural logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = f32::E;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.ln(), 1.0);
    /// assert_eq!(y.ln(), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ln(self) -> Self;

    /// Returns the base 2 logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.log2(), 4.0);
    /// assert_eq!(y.log2(), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 100.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.log10(), 2.0);
    /// assert_eq!(y.log10(), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log10(self) -> Self;

    /// Returns the cube root of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 27.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(x.cbrt(), 3.0);
    /// assert_eq!(y.cbrt(), 0.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "cubeRoot")]
    fn cbrt(self) -> Self;

    /// Returns the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, computes the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and `y.abs()`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// // sqrt(3^2 + 4^2) = sqrt(25) = 5
    /// assert_eq!(x.hypot(y), 5.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn hypot(self, other: Self) -> Self;

    /// Returns the sine of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = core::f32::consts::FRAC_PI_2;
    ///     
    /// assert_eq!(x.sin(), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin(self) -> Self;

    /// Returns the cosine of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert_eq!(x.cos(), -1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cos(self) -> Self;

    /// Returns the tangent of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    ///
    /// assert_eq!(x.tan(), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tan(self) -> Self;

    /// Returns the arcsine of `self` (in radians).
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(x.asin(), core::f32::consts::FRAC_PI_2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asin(self) -> Self;

    /// Returns the arccosine of `self` (in radians).
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[0.0, pi]`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 0.0;
    ///
    /// assert_eq!(x.acos(), core::f32::consts::FRAC_PI_2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acos(self) -> Self;

    /// Returns the arctangent of `self` (in radians).
    ///
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(x.atan(), core::f32::consts::FRAC_PI_4);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan(self) -> Self;

    /// Returns the four-quadrant arctangent of `self` (`y`) and `other` (`x`) (in radians).
    ///
    /// | `x`     | `y`     | Piecewise Definition | Range         |
    /// |---------|---------|----------------------|---------------|
    /// | `>= +0` | `>= +0` | `arctan(y/x)`        | `[+0, +pi/2]` |
    /// | `>= +0` | `<= -0` | `arctan(y/x)`        | `[-pi/2, -0]` |
    /// | `<= -0` | `>= +0` | `arctan(y/x) + pi`   | `[+pi/2, +pi]`|
    /// | `<= -0` | `<= -0` | `arctan(y/x) - pi`   | `[-pi, -pi/2]`|
    ///
    /// # Example
    ///
    /// ```
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1: f32 = 3.0;
    /// let y1: f32 = -3.0;
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2: f32 = -3.0;
    /// let y2: f32 = 3.0;
    ///
    /// let abs_difference_1 = (y1.atan2(x1) - (-core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * core::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= 1e-5);
    /// assert!(abs_difference_2 <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of `self` (in radians),
    /// and returns the result as a tuple `(sin(self), cos(self))`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    /// let (s, c) = x.sin_cos();
    ///
    /// let abs_difference_s = (s - core::f32::consts::FRAC_PI_4.sin()).abs();
    /// let abs_difference_c = (c - core::f32::consts::FRAC_PI_4.cos()).abs();
    ///
    /// assert!(abs_difference_s <= 1e-5);
    /// assert!(abs_difference_c <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin_cos(self) -> (Self, Self);

    /// Returns the hyperbolic sine of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.sinh();
    ///
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sinh(self) -> Self;

    /// Returns the hyperbolic cosine of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.cosh();
    ///
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// // Same result
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cosh(self) -> Self;

    /// Returns the hyperbolic tangent of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.tanh();
    ///
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tanh(self) -> Self;

    /// Returns the inverse hyperbolic sine of `self` (in radians).
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asinh(self) -> Self;

    /// Returns the inverse hyperbolic cosine of `self` (in radians).
    ///
    /// `self` must be greater than or equal to 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acosh(self) -> Self;

    /// Returns the inverse hyperbolic tangent of `self` (in radians).
    ///
    /// The input must be in the range `(-1.0, 1.0)`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 0.5;
    /// let f = x.tanh().atanh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atanh(self) -> Self;

    /// Returns the multiplicative inverse of `self`, `1.0 / self`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 4.0;
    /// let y: f32 = 0.5;
    ///
    /// assert_eq!(x.recip(), 0.25);
    /// assert_eq!(y.recip(), 2.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn recip(self) -> Self;

    /// Converts an angle in radians to degrees.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert_eq!(x.to_degrees(), 180.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_degrees(self) -> Self;

    /// Converts an angle in degrees to radians.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 180.0;
    ///
    /// assert_eq!(x.to_radians(), core::f32::consts::PI);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_radians(self) -> Self;

    /// Returns the midpoint (average) between `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.0;
    /// let y: f32 = 5.0;
    ///
    /// assert_eq!(x.midpoint(y), 4.0);
    /// assert_eq!(y.midpoint(x), 4.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint(self, other: Self) -> Self;

    /// Returns the minimum of `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// assert_eq!(x.min(y), 3.0);
    /// assert_eq!(y.min(x), 3.0);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn min(self, other: Self) -> Self;

    /// Returns the maximum of `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// assert_eq!(x.max(y), 4.0);
    /// assert_eq!(y.max(x), 4.0);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn max(self, other: Self) -> Self;

    /// Restricts `self` to the range defined by `min` and `max`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 15.0;
    /// let min: f32 = 0.0;
    /// let max: f32 = 10.0;
    ///
    /// assert_eq!(x.clamp(min, max), 10.0);
    /// ```
    ////
    /// The following panics because `min > max`:
    ///
    /// ```should_panic
    /// let x: f32 = 15.0;
    /// let min: f32 = 10.0;
    /// let max: f32 = 0.0;
    ///
    /// let _ = x.clamp(min, max);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Returns a number with the magnitude of `self` and the sign of `sign`.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 3.0;
    /// let y: f32 = -2.0;
    ///
    /// assert_eq!(x.copysign(y), -3.0);
    /// assert_eq!(y.copysign(x), 2.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn copysign(self, sign: Self) -> Self;
}

macro_rules! impl_real {
    ($($float:ty),*) => {
        $(
            impl Real for $float {
                #[inline]
                fn floor(self) -> Self {
                    self.floor()
                }
                #[inline]
                fn ceil(self) -> Self {
                    self.ceil()
                }
                #[inline]
                fn round(self) -> Self {
                    self.round()
                }
                #[inline]
                fn trunc(self) -> Self {
                    self.trunc()
                }
                #[inline]
                fn fract(self) -> Self {
                    self.fract()
                }
                #[inline]
                fn div_euclid(self, rhs: Self) -> Self {
                    self.div_euclid(rhs)
                }
                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self {
                    self.rem_euclid(rhs)
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
                fn exp2(self) -> Self {
                    self.exp2()
                }
                #[inline]
                fn log(self, base: Self) -> Self {
                    self.log(base)
                }
                #[inline]
                fn ln(self) -> Self {
                    self.ln()
                }
                #[inline]
                fn log2(self) -> Self {
                    self.log2()
                }
                #[inline]
                fn log10(self) -> Self {
                    self.log10()
                }
                #[inline]
                fn cbrt(self) -> Self {
                    self.cbrt()
                }
                #[inline]
                fn hypot(self, other: Self) -> Self {
                    self.hypot(other)
                }
                #[inline]
                fn sin(self) -> Self {
                    self.sin()
                }
                #[inline]
                fn cos(self) -> Self {
                    self.cos()
                }
                #[inline]
                fn tan(self) -> Self {
                    self.tan()
                }
                #[inline]
                fn asin(self) -> Self {
                    self.asin()
                }
                #[inline]
                fn acos(self) -> Self {
                    self.acos()
                }
                #[inline]
                fn atan(self) -> Self {
                    self.atan()
                }
                #[inline]
                fn atan2(self, other: Self) -> Self {
                    self.atan2(other)
                }
                #[inline]
                fn sin_cos(self) -> (Self, Self) {
                    self.sin_cos()
                }
                #[inline]
                fn sinh(self) -> Self {
                    self.sinh()
                }
                #[inline]
                fn cosh(self) -> Self {
                    self.cosh()
                }
                #[inline]
                fn tanh(self) -> Self {
                    self.tanh()
                }
                #[inline]
                fn asinh(self) -> Self {
                    self.asinh()
                }
                #[inline]
                fn acosh(self) -> Self {
                    self.acosh()
                }
                #[inline]
                fn atanh(self) -> Self {
                    self.atanh()
                }
                #[inline]
                fn recip(self) -> Self {
                    self.recip()
                }
                #[inline]
                fn to_degrees(self) -> Self {
                    self.to_degrees()
                }
                #[inline]
                fn to_radians(self) -> Self {
                    self.to_radians()
                }
                #[inline]
                fn midpoint(self, other: Self) -> Self {
                    self.midpoint(other)
                }
                /// Returns the minimum of `self` and `other`, ignoring NaN values.
                ///
                /// # Example
                ///
                /// ```
                /// let x: f32 = 3.0;
                /// let y: f32 = 4.0;
                /// let nan: f32 = f32::NAN;
                ///
                /// assert_eq!(x.min(y), 3.0);
                /// assert_eq!(x.min(nan), 3.0);
                /// assert_eq!(nan.min(x), 3.0);
                /// ```
                #[inline]
                fn min(self, other: Self) -> Self {
                    self.min(other)
                }
                /// Returns the maximum of `self` and `other`, ignoring NaN values.
                ///
                /// # Example
                ///
                /// ```
                /// let x: f32 = 3.0;
                /// let y: f32 = 4.0;
                /// let nan: f32 = f32::NAN;
                ///
                /// assert_eq!(x.max(y), 4.0);
                /// assert_eq!(x.max(nan), 3.0);
                /// assert_eq!(nan.max(x), 3.0);
                /// ```
                #[inline]
                fn max(self, other: Self) -> Self {
                    self.max(other)
                }
                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }
                #[inline]
                fn copysign(self, sign: Self) -> Self {
                    self.copysign(sign)
                }
            }
        )*
    };
}

impl_real!(f32, f64);

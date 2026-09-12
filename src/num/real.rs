use crate::{
    cmp::NumOrd,
    num::{Int, Num, Signed},
};

/// Basic mathematical constants for [`Real`] number types such as [`f32`] and [`f64`].
pub trait RealConstants {
    /// 1/2
    const HALF: Self;
    /// Euler's number (e)
    const E: Self;
    /// The Euler-Mascheroni constant (γ)
    const EULER_GAMMA: Self;
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
    /// The golden ratio (φ)
    const GOLDEN_RATIO: Self;
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
}

macro_rules! impl_real_constants {
    ($($real:tt),*) => {
        $(
            impl RealConstants for $real {
                const HALF: Self = 0.5;
                const E: Self = core::$real::consts::E;
                const EULER_GAMMA: Self = core::$real::consts::EULER_GAMMA;
                const FRAC_1_PI: Self = core::$real::consts::FRAC_1_PI;
                const FRAC_1_SQRT_2: Self = core::$real::consts::FRAC_1_SQRT_2;
                const FRAC_2_PI: Self = core::$real::consts::FRAC_2_PI;
                const FRAC_2_SQRT_PI: Self = core::$real::consts::FRAC_2_SQRT_PI;
                const FRAC_PI_2: Self = core::$real::consts::FRAC_PI_2;
                const FRAC_PI_3: Self = core::$real::consts::FRAC_PI_3;
                const FRAC_PI_4: Self = core::$real::consts::FRAC_PI_4;
                const FRAC_PI_6: Self = core::$real::consts::FRAC_PI_6;
                const FRAC_PI_8: Self = core::$real::consts::FRAC_PI_8;
                const GOLDEN_RATIO: Self = core::$real::consts::GOLDEN_RATIO;
                const LN_2: Self = core::$real::consts::LN_2;
                const LN_10: Self = core::$real::consts::LN_10;
                const LOG2_10: Self = core::$real::consts::LOG2_10;
                const LOG2_E: Self = core::$real::consts::LOG2_E;
                const LOG10_2: Self = core::$real::consts::LOG10_2;
                const LOG10_E: Self = core::$real::consts::LOG10_E;
                const PI: Self = core::$real::consts::PI;
                const SQRT_2: Self = core::$real::consts::SQRT_2;
                const TAU: Self = core::$real::consts::TAU;
            }
        )*
    };
}

impl_real_constants!(f32, f64);

/// A trait for [real number] types such as [`f32`] and [`f64`].
///
/// [real number]: https://en.wikipedia.org/wiki/Real_number
pub trait Real: Num + Signed + RealConstants + NumOrd {
    /// The integer type associated with this real number type.
    type I32: Int;

    /// Creates a real number from an [`f32`] value.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_f32(n: f32) -> Self;

    /// Creates a real number from an [`f64`] value.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_f64(n: f64) -> Self;

    /// Returns the largest integer less than or equal to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(Real::floor(x), 3.0);
    /// assert_eq!(Real::floor(y), -4.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(Real::ceil(x), 4.0);
    /// assert_eq!(Real::ceil(y), -3.0);
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
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = 4.5;
    /// let z: f32 = -5.5;
    ///
    /// assert_eq!(Real::round(x), 4.0);
    /// assert_eq!(Real::round(y), 5.0);
    /// assert_eq!(Real::round(z), -6.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn round(self) -> Self;

    /// Returns the integer nearest to `self`. If a value is half-way
    /// between two integers, rounds to the number with an even
    /// least significant digit.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = 4.5;
    /// let z: f32 = -5.5;
    ///
    /// assert_eq!(Real::round_ties_even(x), 4.0);
    /// assert_eq!(Real::round_ties_even(y), 4.0);
    /// assert_eq!(Real::round_ties_even(z), -6.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn round_ties_even(self) -> Self;

    /// Returns the integer part of `self`. This means that non-integer numbers
    /// are always truncated towards zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert_eq!(Real::trunc(x), 3.0);
    /// assert_eq!(Real::trunc(y), -3.0);
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
    /// # use gnum::num::Real;
    /// let x: f32 = 3.7;
    /// let y: f32 = -3.7;
    ///
    /// assert!((Real::fract(x) - 0.7).abs() <= f32::EPSILON);
    /// assert!((Real::fract(y) + 0.7).abs() <= f32::EPSILON);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn fract(self) -> Self;

    /// Returns the square root of `self`.
    ///
    /// `self` must be non-negative.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 4.0;
    /// let y: f32 = 2.0;
    ///
    /// assert_eq!(Real::sqrt(x), 2.0);
    /// assert_eq!(Real::sqrt(y), 1.4142135);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "squareRoot")]
    fn sqrt(self) -> Self;

    /// Returns `pow(e, self)`, the exponential function.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`exp_stable`](Self::exp_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::exp(x), core::f32::consts::E);
    /// assert_eq!(Real::exp(y), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp(self) -> Self;

    /// Returns `pow(e, self)`, the exponential function, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`exp`](Self::exp) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::exp_stable(x) - core::f32::consts::E).abs() <= 1e-5);
    /// assert_eq!(Real::exp_stable(y), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp_stable(self) -> Self;

    /// Returns `pow(2, self)`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`exp2_stable`](Self::exp2_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 2.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::exp2(x), 4.0);
    /// assert_eq!(Real::exp2(y), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp2(self) -> Self;

    /// Returns `pow(2, self)`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`exp2`](Self::exp2) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 2.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::exp2_stable(x) - 4.0).abs() <= 1e-5);
    /// assert_eq!(Real::exp2_stable(y), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn exp2_stable(self) -> Self;

    /// Returns the logarithm of `self` with respect to a given `base`.
    ///
    /// `self` must be positive.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`log_stable`](Self::log_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let base: f32 = 2.0;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::log(x, base), 4.0);
    /// assert_eq!(Real::log(y, base), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log(self, base: Self) -> Self;

    /// Returns the logarithm of `self` with respect to a given `base`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`log`](Self::log) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let base: f32 = 2.0;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::log_stable(x, base) - 4.0).abs() <= 1e-5);
    /// assert_eq!(Real::log_stable(y, base), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log_stable(self, base: Self) -> Self;

    /// Returns the natural logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`ln_stable`](Self::ln_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::E;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::ln(x) - 1.0).abs() <= f32::EPSILON);
    /// assert_eq!(Real::ln(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ln(self) -> Self;

    /// Returns the natural logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`ln`](Self::ln) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::E;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::ln_stable(x) - 1.0).abs() <= 1e-5);
    /// assert_eq!(Real::ln_stable(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ln_stable(self) -> Self;

    /// Returns the base 2 logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`log2_stable`](Self::log2_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::log2(x), 4.0);
    /// assert_eq!(Real::log2(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log2(self) -> Self;

    /// Returns the base 2 logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`log2`](Self::log2) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::log2_stable(x) - 4.0).abs() <= 1e-5);
    /// assert_eq!(Real::log2_stable(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log2_stable(self) -> Self;

    /// Returns the base 10 logarithm of `self`.
    ///
    /// `self` must be positive.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`log10_stable`](Self::log10_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 100.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::log10(x), 2.0);
    /// assert_eq!(Real::log10(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log10(self) -> Self;

    /// Returns the base 10 logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`log10`](Self::log10) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 100.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::log10_stable(x) - 2.0).abs() <= 1e-5);
    /// assert_eq!(Real::log10_stable(y), f32::NEG_INFINITY);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn log10_stable(self) -> Self;

    /// Returns the cube root of `self`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`cbrt_stable`](Self::cbrt_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 27.0;
    /// let y: f32 = 0.0;
    ///
    /// assert_eq!(Real::cbrt(x), 3.0);
    /// assert_eq!(Real::cbrt(y), 0.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "cubeRoot")]
    fn cbrt(self) -> Self;

    /// Returns the cube root of `self`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`cbrt`](Self::cbrt) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 27.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((Real::cbrt_stable(x) - 3.0).abs() <= 1e-4);
    /// assert_eq!(Real::cbrt_stable(y), 0.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "cubeRootStable")]
    fn cbrt_stable(self) -> Self;

    /// Returns the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, computes the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and `y.abs()`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`hypot_stable`](Self::hypot_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// // sqrt(3^2 + 4^2) = sqrt(25) = 5
    /// assert_eq!(Real::hypot(x, y), 5.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn hypot(self, other: Self) -> Self;

    /// Returns the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane, with deterministic results. Equivalently, computes
    /// the length of the hypotenuse of a right-angle triangle with other sides
    /// having length `x.abs()` and `y.abs()`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`hypot`](Self::hypot) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// // sqrt(3^2 + 4^2) = sqrt(25) = 5
    /// assert_eq!(Real::hypot_stable(x, y), 5.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn hypot_stable(self, other: Self) -> Self;

    /// Returns the sine of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`sin_stable`](Self::sin_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_2;
    ///     
    /// assert_eq!(Real::sin(x), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin(self) -> Self;

    /// Returns the sine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`sin`](Self::sin) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_2;
    ///
    /// assert!((Real::sin_stable(x) - 1.0).abs() <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin_stable(self) -> Self;

    /// Returns the cosine of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`cos_stable`](Self::cos_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert_eq!(Real::cos(x), -1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cos(self) -> Self;

    /// Returns the cosine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`cos`](Self::cos) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert!((Real::cos_stable(x) - (-1.0)).abs() <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cos_stable(self) -> Self;

    /// Returns the tangent of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`tan_stable`](Self::tan_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    ///
    /// assert_eq!(Real::tan(x), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tan(self) -> Self;

    /// Returns the tangent of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`tan`](Self::tan) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    ///
    /// assert!((Real::tan_stable(x) - 1.0).abs() <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tan_stable(self) -> Self;

    /// Returns the arcsine of `self` (in radians).
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`asin_stable`](Self::asin_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(Real::asin(x), core::f32::consts::FRAC_PI_2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asin(self) -> Self;

    /// Returns the arcsine of `self` (in radians), with deterministic results.
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`asin`](Self::asin) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(Real::asin_stable(x), core::f32::consts::FRAC_PI_2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asin_stable(self) -> Self;

    /// Returns the arccosine of `self` (in radians).
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[0.0, pi]`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`acos_stable`](Self::acos_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 0.0;
    ///
    /// assert_eq!(Real::acos(x), core::f32::consts::FRAC_PI_2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acos(self) -> Self;

    /// Returns the arccosine of `self` (in radians), with deterministic results.
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[0.0, pi]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`acos`](Self::acos) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 0.0;
    ///
    /// assert!((Real::acos_stable(x) - core::f32::consts::FRAC_PI_2).abs() <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acos_stable(self) -> Self;

    /// Returns the arctangent of `self` (in radians).
    ///
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`atan_stable`](Self::atan_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(Real::atan(x), core::f32::consts::FRAC_PI_4);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan(self) -> Self;

    /// Returns the arctangent of `self` (in radians), with deterministic results.
    ///
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`atan`](Self::atan) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    ///
    /// assert!((Real::atan_stable(x) - core::f32::consts::FRAC_PI_4).abs() <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan_stable(self) -> Self;

    /// Returns the four-quadrant arctangent of `self` (`y`) and `other` (`x`) (in radians).
    ///
    /// | `x`     | `y`     | Piecewise Definition | Range         |
    /// |---------|---------|----------------------|---------------|
    /// | `>= +0` | `>= +0` | `arctan(y/x)`        | `[+0, +pi/2]` |
    /// | `>= +0` | `<= -0` | `arctan(y/x)`        | `[-pi/2, -0]` |
    /// | `<= -0` | `>= +0` | `arctan(y/x) + pi`   | `[+pi/2, +pi]`|
    /// | `<= -0` | `<= -0` | `arctan(y/x) - pi`   | `[-pi, -pi/2]`|
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`atan2_stable`](Self::atan2_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// // Positive angles measured counterclockwise from the positive x axis
    ///
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1: f32 = 3.0;
    /// let y1: f32 = -3.0;
    ///
    /// // 3pi/4 radians (135 deg counterclockwise)
    /// let x2: f32 = -3.0;
    /// let y2: f32 = 3.0;
    ///
    /// let abs_difference_1 = (Real::atan2(y1, x1) - (-core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (Real::atan2(y2, x2) - (3.0 * core::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= 1e-5);
    /// assert!(abs_difference_2 <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan2(self, other: Self) -> Self;

    /// Returns the four-quadrant arctangent of `self` (`y`) and `other` (`x`) (in radians),
    /// with deterministic results.
    ///
    /// | `x`     | `y`     | Piecewise Definition | Range         |
    /// |---------|---------|----------------------|---------------|
    /// | `>= +0` | `>= +0` | `arctan(y/x)`        | `[+0, +pi/2]` |
    /// | `>= +0` | `<= -0` | `arctan(y/x)`        | `[-pi/2, -0]` |
    /// | `<= -0` | `>= +0` | `arctan(y/x) + pi`   | `[+pi/2, +pi]`|
    /// | `<= -0` | `<= -0` | `arctan(y/x) - pi`   | `[-pi, -pi/2]`|
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`atan2`](Self::atan2) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// // Positive angles measured counterclockwise from the positive x axis
    ///
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1: f32 = 3.0;
    /// let y1: f32 = -3.0;
    ///
    /// // 3pi/4 radians (135 deg counterclockwise)
    /// let x2: f32 = -3.0;
    /// let y2: f32 = 3.0;
    ///
    /// let abs_difference_1 = (Real::atan2_stable(y1, x1) - (-core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (Real::atan2_stable(y2, x2) - (3.0 * core::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_1 <= 1e-5);
    /// assert!(abs_difference_2 <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atan2_stable(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of `self` (in radians),
    /// and returns the result as a tuple `(sin(self), cos(self))`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`sin_cos_stable`](Self::sin_cos_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    /// let (s, c) = Real::sin_cos(x);
    ///
    /// let abs_difference_s = (s - Real::sin(core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_c = (c - Real::cos(core::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_s <= 1e-5);
    /// assert!(abs_difference_c <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin_cos(self) -> (Self, Self);

    /// Simultaneously computes the sine and cosine of `self` (in radians),
    /// and returns the result as a tuple `(sin(self), cos(self))`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`sin_cos`](Self::sin_cos) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    /// let (s, c) = Real::sin_cos_stable(x);
    ///
    /// let abs_difference_s = (s - Real::sin(core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_c = (c - Real::cos(core::f32::consts::FRAC_PI_4)).abs();
    ///
    /// assert!(abs_difference_s <= 1e-5);
    /// assert!(abs_difference_c <= 1e-5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sin_cos_stable(self) -> (Self, Self);

    /// Returns the hyperbolic sine of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`sinh_stable`](Self::sinh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::sinh(x);
    ///
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sinh(self) -> Self;

    /// Returns the hyperbolic sine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`sinh`](Self::sinh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::sinh_stable(x);
    ///
    /// // Solving sinh() at 1 gives `(e^2-1)/(2e)`
    /// let g = ((e * e) - 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn sinh_stable(self) -> Self;

    /// Returns the hyperbolic cosine of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`cosh_stable`](Self::cosh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::cosh(x);
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

    /// Returns the hyperbolic cosine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`cosh`](Self::cosh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::cosh_stable(x);
    ///
    /// // Solving cosh() at 1 gives this result
    /// let g = ((e * e) + 1.0) / (2.0 * e);
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cosh_stable(self) -> Self;

    /// Returns the hyperbolic tangent of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`tanh_stable`](Self::tanh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::tanh(x);
    ///
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tanh(self) -> Self;

    /// Returns the hyperbolic tangent of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`tanh`](Self::tanh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = Real::tanh_stable(x);
    ///
    /// // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
    /// let g = (1.0 - e.powi(-2)) / (1.0 + e.powi(-2));
    /// let abs_difference = (f - g).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn tanh_stable(self) -> Self;

    /// Returns the inverse hyperbolic sine of `self` (in radians).
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`asinh_stable`](Self::asinh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let f = Real::asinh(Real::sinh(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asinh(self) -> Self;

    /// Returns the inverse hyperbolic sine of `self` (in radians),
    /// with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`asinh`](Self::asinh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let f = Real::asinh_stable(Real::sinh_stable(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn asinh_stable(self) -> Self;

    /// Returns the inverse hyperbolic cosine of `self` (in radians).
    ///
    /// `self` must be greater than or equal to `1.0`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`acosh_stable`](Self::acosh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let f = Real::acosh(Real::cosh(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acosh(self) -> Self;

    /// Returns the inverse hyperbolic cosine of `self` (in radians),
    /// with deterministic results.
    ///
    /// `self` must be greater than or equal to `1.0`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`acosh`](Self::acosh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 1.0;
    /// let f = Real::acosh_stable(Real::cosh_stable(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn acosh_stable(self) -> Self;

    /// Returns the inverse hyperbolic tangent of `self` (in radians).
    ///
    /// The input must be in the range `(-1.0, 1.0)`.
    ///
    /// # Unspecified Precision
    ///
    /// For some types (ex: floating-point numbers), the precision of this function
    /// is non-deterministic. This means it varies by platform, Rust version, and can
    /// even differ within the same execution from one invocation to the next.
    ///
    /// See [`atanh_stable`](Self::atanh_stable) for a version of this function
    /// that is guaranteed to be deterministic and returns identical results
    /// across both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 0.5;
    /// let f = Real::atanh(Real::tanh(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atanh(self) -> Self;

    /// Returns the inverse hyperbolic tangent of `self` (in radians),
    ///
    /// The input must be in the range `(-1.0, 1.0)`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some precision
    /// and/or performance.
    ///
    /// See [`atanh`](Self::atanh) for a version of this function that may be
    /// more precise but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 0.5;
    /// let f = Real::atanh_stable(Real::tanh_stable(x));
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= 1e-6);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn atanh_stable(self) -> Self;

    /// Returns the multiplicative inverse of `self`, `1.0 / self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 4.0;
    /// let y: f32 = 0.5;
    ///
    /// assert_eq!(Real::recip(x), 0.25);
    /// assert_eq!(Real::recip(y), 2.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn recip(self) -> Self;

    /// Converts an angle in radians to degrees.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert_eq!(Real::to_degrees(x), 180.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_degrees(self) -> Self;

    /// Converts an angle in degrees to radians.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 180.0;
    ///
    /// assert_eq!(Real::to_radians(x), core::f32::consts::PI);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_radians(self) -> Self;

    /// Returns the midpoint (average) between `self` and `other`.
    ///
    /// # NaN Behavior
    ///
    /// If the result is NaN, its sign and payload may differ between scalar and SIMD
    /// implementations. Rust leaves those bits unspecified.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.0;
    /// let y: f32 = 5.0;
    ///
    /// assert_eq!(Real::midpoint(x, y), 4.0);
    /// assert_eq!(Real::midpoint(y, x), 4.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint(self, other: Self) -> Self;

    /// Returns the midpoint (average) between `self` and `other`, computed as `(self + other) / 2`.
    ///
    /// This is a faster version of [`midpoint`](Self::midpoint), which additionally guards
    /// against overflow and underflow. For example, `f32::MAX.midpoint(f32::MAX)` is `f32::MAX`,
    /// but `f32::MAX.midpoint_fast(f32::MAX)` overflows to `f32::INFINITY`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// assert_eq!(Real::midpoint_fast(3.0f32, 5.0), 4.0);
    ///
    /// // `midpoint_fast` can overflow, while `midpoint` does not
    /// assert_eq!(Real::midpoint_fast(f32::MAX, f32::MAX), f32::INFINITY);
    /// assert_eq!(Real::midpoint(f32::MAX, f32::MAX), f32::MAX);
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint_fast(self, other: Self) -> Self {
        (self + other) * Self::HALF
    }

    /// Returns a number with the magnitude of `self` and the sign of `sign`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// let x: f32 = 3.0;
    /// let y: f32 = -2.0;
    ///
    /// assert_eq!(Real::copysign(x, y), -3.0);
    /// assert_eq!(Real::copysign(y, x), 2.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn copysign(self, sign: Self) -> Self;
}

macro_rules! impl_real {
    ($($float:ty),*) => {
        $(
            impl Real for $float {
                type I32 = i32;

                #[inline]
                fn from_f32(n: f32) -> Self {
                    n as Self
                }
                #[inline]
                fn from_f64(n: f64) -> Self {
                    n as Self
                }
                #[inline]
                fn floor(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.floor()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::round::RoundOps::floor_internal(self)
                    }
                }
                #[inline]
                fn ceil(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.ceil()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::round::RoundOps::ceil_internal(self)
                    }
                }
                #[inline]
                fn round(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.round()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::round::RoundOps::round_internal(self)
                    }
                }
                #[inline]
                fn round_ties_even(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.round_ties_even()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::round::RoundOps::round_ties_even_internal(self)
                    }
                }
                #[inline]
                fn trunc(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.trunc()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::round::RoundOps::trunc_internal(self)
                    }
                }
                #[inline]
                fn fract(self) -> Self {
                    self - Real::trunc(self)
                }
                #[inline]
                fn sqrt(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.sqrt()
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        crate::num::stable::sqrt(self)
                    }
                }
                #[inline]
                fn exp(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        self.exp()
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
                        self.exp2()
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
                        self.log(base)
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
                        self.ln()
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
                        self.log2()
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
                        self.log10()
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
                        self.cbrt()
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
                        self.hypot(other)
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
                        self.sin()
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
                        self.cos()
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
                        self.tan()
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
                        self.asin()
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
                        self.acos()
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
                        self.atan()
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
                        self.atan2(other)
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
                        self.sin_cos()
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
                        self.sinh()
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
                        self.cosh()
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
                        self.tanh()
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
                        self.asinh()
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
                        self.acosh()
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
                        self.atanh()
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
                #[inline]
                fn copysign(self, sign: Self) -> Self {
                    self.copysign(sign)
                }
            }
        )*
    };
}

impl_real!(f32, f64);

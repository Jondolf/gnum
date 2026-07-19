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
/// [real numbers]: https://en.wikipedia.org/wiki/Real_number
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
    /// let y: f32 = 4.5;
    /// let z: f32 = -5.5;
    ///
    /// assert_eq!(x.round(), 4.0);
    /// assert_eq!(y.round(), 5.0);
    /// assert_eq!(z.round(), -6.0);
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
    /// let x: f32 = 3.7;
    /// let y: f32 = 4.5;
    /// let z: f32 = -5.5;
    ///
    /// assert_eq!(x.round_ties_even(), 4.0);
    /// assert_eq!(y.round_ties_even(), 4.0);
    /// assert_eq!(z.round_ties_even(), -6.0);
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn round_ties_even(self) -> Self;

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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns `pow(e, self)`, the exponential function, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`exp`](Self::exp) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 1.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.exp_stable() - core::f32::consts::E).abs() <= 1e-5);
    /// assert_eq!(y.exp_stable(), 1.0);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns `pow(2, self)`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`exp2`](Self::exp2) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 2.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.exp2_stable() - 4.0).abs() <= 1e-5);
    /// assert_eq!(y.exp2_stable(), 1.0);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the logarithm of `self` with respect to a given `base`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`log`](Self::log) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let base: f32 = 2.0;
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.log_stable(base) - 4.0).abs() <= 1e-5);
    /// assert_eq!(y.log_stable(base), f32::NEG_INFINITY);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the natural logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`ln`](Self::ln) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = core::f32::consts::E;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.ln_stable() - 1.0).abs() <= 1e-5);
    /// assert_eq!(y.ln_stable(), f32::NEG_INFINITY);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the base 2 logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`log2`](Self::log2) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 16.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.log2_stable() - 4.0).abs() <= 1e-5);
    /// assert_eq!(y.log2_stable(), f32::NEG_INFINITY);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the base 10 logarithm of `self`, with deterministic results.
    ///
    /// `self` must be positive.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`log10`](Self::log10) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 100.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.log10_stable() - 2.0).abs() <= 1e-5);
    /// assert_eq!(y.log10_stable(), f32::NEG_INFINITY);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the cube root of `self`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`cbrt`](Self::cbrt) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 27.0;
    /// let y: f32 = 0.0;
    ///
    /// assert!((x.cbrt_stable() - 3.0).abs() <= 1e-4);
    /// assert_eq!(y.cbrt_stable(), 0.0);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane, with deterministic results. Equivalently, computes
    /// the length of the hypotenuse of a right-angle triangle with other sides
    /// having length `x.abs()` and `y.abs()`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`hypot`](Self::hypot) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 3.0;
    /// let y: f32 = 4.0;
    ///
    /// // sqrt(3^2 + 4^2) = sqrt(25) = 5
    /// assert_eq!(x.hypot_stable(y), 5.0);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the sine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`sin`](Self::sin) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = core::f32::consts::FRAC_PI_2;
    ///
    /// assert!((x.sin_stable() - 1.0).abs() <= 1e-6);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the cosine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`cos`](Self::cos) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = core::f32::consts::PI;
    ///
    /// assert!((x.cos_stable() - (-1.0)).abs() <= 1e-6);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the tangent of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`tan`](Self::tan) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    ///
    /// assert!((x.tan_stable() - 1.0).abs() <= 1e-6);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the arcsine of `self` (in radians), with deterministic results.
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`asin`](Self::asin) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 1.0;
    ///
    /// assert_eq!(x.asin_stable(), core::f32::consts::FRAC_PI_2);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the arccosine of `self` (in radians), with deterministic results.
    ///
    /// The input must be in the range `[-1.0, 1.0]`.
    /// The output is in the range `[0.0, pi]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`acos`](Self::acos) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 0.0;
    ///
    /// assert_eq!(x.acos_stable(), core::f32::consts::FRAC_PI_2);
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the arctangent of `self` (in radians), with deterministic results.
    ///
    /// The output is in the range `[-pi/2, pi/2]`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`atan`](Self::atan) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 1.0;
    ///
    /// assert!((x.atan_stable() - core::f32::consts::FRAC_PI_4).abs() <= 1e-5);
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
    /// across both scalar and vectorized types at the cost of some performance.
    ///
    /// # Example
    ///
    /// ```
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
    /// let abs_difference_1 = (y1.atan2(x1) - (-core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2(x2) - (3.0 * core::f32::consts::FRAC_PI_4)).abs();
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
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`atan2`](Self::atan2) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
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
    /// let abs_difference_1 = (y1.atan2_stable(x1) - (-core::f32::consts::FRAC_PI_4)).abs();
    /// let abs_difference_2 = (y2.atan2_stable(x2) - (3.0 * core::f32::consts::FRAC_PI_4)).abs();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Simultaneously computes the sine and cosine of `self` (in radians),
    /// and returns the result as a tuple `(sin(self), cos(self))`, with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`sin_cos`](Self::sin_cos) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = core::f32::consts::FRAC_PI_4;
    /// let (s, c) = x.sin_cos_stable();
    ///
    /// let abs_difference_s = (s - core::f32::consts::FRAC_PI_4.sin()).abs();
    /// let abs_difference_c = (c - core::f32::consts::FRAC_PI_4.cos()).abs();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the hyperbolic sine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`sinh`](Self::sinh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.sinh_stable();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the hyperbolic cosine of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`cosh`](Self::cosh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.cosh_stable();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the hyperbolic tangent of `self` (in radians), with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`tanh`](Self::tanh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let e = core::f32::consts::E;
    /// let x: f32 = 1.0;
    /// let f = x.tanh_stable();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the inverse hyperbolic sine of `self` (in radians),
    /// with deterministic results.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`asinh`](Self::asinh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 1.0;
    /// let f = x.sinh_stable().asinh_stable();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the inverse hyperbolic cosine of `self` (in radians),
    /// with deterministic results.
    ///
    /// `self` must be greater than or equal to `1.0`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`acosh`](Self::acosh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 1.0;
    /// let f = x.cosh_stable().acosh_stable();
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
    /// across both scalar and vectorized types at the cost of some performance.
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

    /// Returns the inverse hyperbolic tangent of `self` (in radians),
    ///
    /// The input must be in the range `(-1.0, 1.0)`.
    ///
    /// # Precision
    ///
    /// This function is deterministic and returns identical results across
    /// both scalar and vectorized types at the cost of some performance.
    ///
    /// See [`atanh`](Self::atanh) for a version of this function that may be faster
    /// but can be non-deterministic.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Real;
    /// #
    /// let x: f32 = 0.5;
    /// let f = x.tanh_stable().atanh_stable();
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

    /// Returns the midpoint (average) between `self` and `other`, computed as `(self + other) / 2`.
    ///
    /// This is a faster version of [`midpoint`](Self::midpoint), which additionally guards
    /// against overflow and underflow. For example, `f32::MAX.midpoint(f32::MAX)` is `f32::MAX`,
    /// but `f32::MAX.midpoint_fast(f32::MAX)` overflows to `f32::INFINITY`.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::num::Real;
    ///
    /// assert_eq!(Real::midpoint_fast(3.0f32, 5.0), 4.0);
    ///
    /// // `midpoint_fast` can overflow, while `midpoint` does not
    /// assert!(Real::midpoint_fast(f32::MAX, f32::MAX).is_infinite());
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
                fn round_ties_even(self) -> Self {
                    self.round_ties_even()
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
                    self.log(base)
                }
                #[inline]
                fn log_stable(self, base: Self) -> Self {
                    crate::num::stable::log(self, base)
                }
                #[inline]
                fn ln(self) -> Self {
                    self.ln()
                }
                #[inline]
                fn ln_stable(self) -> Self {
                    crate::num::stable::ln(self)
                }
                #[inline]
                fn log2(self) -> Self {
                    self.log2()
                }
                #[inline]
                fn log2_stable(self) -> Self {
                    crate::num::stable::log2(self)
                }
                #[inline]
                fn log10(self) -> Self {
                    self.log10()
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
                    self.hypot(other)
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
                    self.tanh()
                }
                #[inline]
                fn tanh_stable(self) -> Self {
                    crate::num::stable::tanh(self)
                }
                #[inline]
                fn asinh(self) -> Self {
                    self.asinh()
                }
                #[inline]
                fn asinh_stable(self) -> Self {
                    crate::num::stable::asinh(self)
                }
                #[inline]
                fn acosh(self) -> Self {
                    self.acosh()
                }
                #[inline]
                fn acosh_stable(self) -> Self {
                    crate::num::stable::acosh(self)
                }
                #[inline]
                fn atanh(self) -> Self {
                    self.atanh()
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

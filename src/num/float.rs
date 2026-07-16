use crate::num::{Int, Real};

/// A trait for floating-point types representing [`Real`] numbers such as [`f32`] and [`f64`].
///
/// Floating-point types are expected to conform to the IEEE 754-2008 standard.
pub trait Float: Real {
    /// The unsigned integer type used as the raw bit representation
    /// of the floating-point type.
    type Bits: Int<Unsigned = Self::Bits>;

    /// The radix or base of the internal representation of the floating-point type.
    const RADIX: u32;

    /// Number of significant digits in base 2.
    ///
    /// Note that the size of the mantissa in the bitwise representation is one
    /// smaller than this since the leading 1 is not stored explicitly.
    const MANTISSA_DIGITS: u32;

    /// Approximate number of significant digits in base 10.
    ///
    /// This is the maximum <i>x</i> such that any decimal number with <i>x</i> significant digits
    /// can be converted to the floating-point type and back without loss.
    ///
    /// Equal to floor(log<sub>10</sub>&nbsp;2<sup>[`MANTISSA_DIGITS`]&nbsp;&minus;&nbsp;1</sup>).
    ///
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    const DIGITS: u32;

    /// [Machine epsilon] value for the floating-point type.
    ///
    /// This is the difference between `1.0` and the next larger representable number.
    ///
    /// Equal to 2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    const EPSILON: Self;

    /// Smallest positive normal value for the floating-point type.
    ///
    /// Equal to 2<sup>[`MIN_EXP`]&nbsp;&minus;&nbsp;1</sup>.
    ///
    /// [`MIN_EXP`]: Self::MIN_EXP
    const MIN_POSITIVE: Self;

    /// One greater than the minimum possible *normal* power of 2 exponent
    /// for a significand bounded by 1 ≤ x < 2 (i.e. the IEEE definition).
    ///
    /// This corresponds to the exact minimum possible *normal* power of 2 exponent
    /// for a significand bounded by 0.5 ≤ x < 1 (i.e. the C definition).
    /// In other words, all normal numbers representable by this type are
    /// greater than or equal to 0.5&nbsp;×&nbsp;2<sup><i>MIN_EXP</i></sup>.
    const MIN_EXP: i32;

    /// One greater than the maximum possible power of 2 exponent
    /// for a significand bounded by 1 ≤ x < 2 (i.e. the IEEE definition).
    ///
    /// This corresponds to the exact maximum possible power of 2 exponent
    /// for a significand bounded by 0.5 ≤ x < 1 (i.e. the C definition).
    /// In other words, all numbers representable by this type are
    /// strictly less than 2<sup><i>MAX_EXP</i></sup>.
    const MAX_EXP: i32;

    /// Minimum <i>x</i> for which 10<sup><i>x</i></sup> is normal.
    ///
    /// Equal to ceil(log<sub>10</sub>&nbsp;[`MIN_POSITIVE`]).
    ///
    /// [`MIN_POSITIVE`]: Self::MIN_POSITIVE
    const MIN_10_EXP: i32;

    /// Maximum <i>x</i> for which 10<sup><i>x</i></sup> is normal.
    ///
    /// Equal to floor(log<sub>10</sub>&nbsp;[`MAX`]).
    ///
    /// [`MAX`]: Self::MAX
    const MAX_10_EXP: i32;

    /// Not a Number (NaN).
    ///
    /// Note that IEEE 754 doesn't define just a single NaN value; a plethora of bit patterns are
    /// considered to be NaN. Furthermore, the standard makes a difference between a "signaling" and
    /// a "quiet" NaN, and allows inspecting its "payload" (the unspecified bits in the bit pattern)
    /// and its sign. See the [specification of NaN bit patterns](f32#nan-bit-patterns) for more
    /// info.
    ///
    /// This constant is guaranteed to be a quiet NaN (on targets that follow the Rust assumptions
    /// that the quiet/signaling bit being set to 1 indicates a quiet NaN). Beyond that, nothing is
    /// guaranteed about the specific bit pattern chosen here: both payload and sign are arbitrary.
    /// The concrete bit pattern may change across Rust versions and target platforms.
    const NAN: Self;

    /// Infinity (∞).
    const INFINITY: Self;

    /// Negative infinity (−∞).
    const NEG_INFINITY: Self;

    /// Returns the result of a fused multiply-add `(self * a) + b` with only one rounding error,
    /// yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` may be more performant than an unfused multiply-add if the target architecture
    /// has a dedicated `fma` CPU instruction. However, this is not always true, and will be heavily dependant
    /// on designing algorithms with specific target hardware in mind.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f64 = 10.0;
    /// let a: f64 = 4.0;
    /// let b: f64 = 60.0;
    ///
    /// assert_eq!(x.mul_add(a, b), 100.0);
    /// assert_eq!(x * a + b, 100.0);
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "fmaf", alias = "fusedMultiplyAdd")]
    fn mul_add(self, a: Self, b: Self) -> Self;

    /* TODO: Some number backends might not support integers properly
    /// Returns `self` raised to the integer power of `n`.
    ///
    /// Using this function is generally faster than using [`powf`](Self::powf).
    /// It might have a different sequence of rounding operations than [`powf`](Self::powf),
    /// so the results are not guaranteed to agree.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: f32 = 2.0;
    /// let abs_difference = (x.powi(2) - (x * x)).abs();
    /// assert!(abs_difference <= 1e-5);
    ///
    /// assert_eq!(f32::powi(f32::NAN, 0), 1.0);
    /// assert_eq!(f32::powi(0.0, 0), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn powi(self, n: Self::Int) -> Self;
    */

    /// Returns `self` raised to the power of `n`.
    ///
    /// # Examples
    ///
    /// ```
    /// let x: f32 = 2.0;
    /// let abs_difference = (x.powf(2.0) - (x * x)).abs();
    /// assert!(abs_difference <= 1e-5);
    ///
    /// assert_eq!(f32::powf(1.0, f32::NAN), 1.0);
    /// assert_eq!(f32::powf(f32::NAN, 0.0), 1.0);
    /// assert_eq!(f32::powf(0.0, 0.0), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn powf(self, n: Self) -> Self;

    /// Returns `true` if `self` is NaN, and `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let nan: f32 = f32::NAN;
    /// let x: f32 = 1.0;
    ///
    /// assert!(nan.is_nan());
    /// assert!(!x.is_nan());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_nan(self) -> Self::Bool;

    /// Returns `true` if `self` is positive or negative infinity, and `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let inf: f32 = f32::INFINITY;
    /// let neg_inf: f32 = f32::NEG_INFINITY;
    /// let x: f32 = 1.0;
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// assert!(!x.is_infinite());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_infinite(self) -> Self::Bool;

    /// Returns `true` if `self` is neither infinite nor NaN, and `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    /// let inf: f32 = f32::INFINITY;
    /// let neg_inf: f32 = f32::NEG_INFINITY;
    /// let nan: f32 = f32::NAN;
    ///
    /// assert!(x.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// assert!(!nan.is_finite());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_finite(self) -> Self::Bool;

    /// Returns `true` if `self` is a [subnormal] number, and `false` otherwise.
    ///
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    ///
    /// # Example
    ///
    /// ```
    /// let min: f32 = f32::MIN_POSITIVE;
    /// let max: f32 = f32::MAX;
    /// let lower_than_min: f32 = 1e-40;
    /// let zero: f32 = 0.0;
    ///
    /// assert!(!min.is_subnormal());
    /// assert!(!max.is_subnormal());
    ///
    /// assert!(!zero.is_subnormal());
    /// assert!(!f32::NAN.is_subnormal());
    /// assert!(!f32::INFINITY.is_subnormal());
    /// assert!(lower_than_min.is_subnormal());
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_subnormal(self) -> Self::Bool;

    /// Returns `true` if `self` is neither zero, infinite, [subnormal], nor NaN, and `false` otherwise.
    ///
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    ///
    /// # Example
    ///
    /// ```
    /// let min: f32 = f32::MIN_POSITIVE;
    /// let max: f32 = f32::MAX;
    /// let lower_than_min: f32 = 1e-40;
    /// let zero: f32 = 0.0;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!f32::NAN.is_normal());
    /// assert!(!f32::INFINITY.is_normal());
    /// assert!(!lower_than_min.is_normal());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_normal(self) -> Self::Bool;

    /// Returns `true` if `self` has a positive sign, including `+0.0`, NaNs with
    /// a positive sign bit, and positive infinity.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 7.0;
    /// let y: f32 = -7.0;
    ///
    /// assert!(x.is_sign_positive());
    /// assert!(!y.is_sign_positive());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_sign_positive(self) -> Self::Bool;

    /// Returns `true` if `self` has a negative sign, including `-0.0`, NaNs with
    /// a negative sign bit, and negative infinity.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 7.0;
    /// let y: f32 = -7.0;
    ///
    /// assert!(!x.is_sign_negative());
    /// assert!(y.is_sign_negative());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_sign_negative(self) -> Self::Bool;

    /// Returns the least number greater than `self`.
    ///
    /// This corresponds to the IEEE 754 `nextUp` operation.
    ///
    /// # Example
    ///
    /// ```
    /// // f32::EPSILON is the difference between 1.0 and the next number up.
    /// assert_eq!(1.0.next_up(), 1.0 + f32::EPSILON);
    /// // But not for most numbers.
    /// assert!(0.1.next_up() < 0.1 + f32::EPSILON);
    /// assert_eq!(16777216.0.next_up(), 16777218.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn next_up(self) -> Self;

    /// Returns the greatest number less than `self`.
    ///
    /// This corresponds to the IEEE 754 `nextDown` operation.
    ///
    /// # Example
    ///
    /// ```
    /// let x: f32 = 1.0;
    /// // Clamp value into range [0, 1).
    /// let clamped = x.clamp(0.0, 1.0.next_down());
    /// assert!(clamped < 1.0);
    /// assert_eq!(clamped.next_up(), 1.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn next_down(self) -> Self;

    /// Raw transmutation from an unsigned integer to a floating-point type.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Float;
    /// let x: f32 = 2.5;
    /// let bits: u32 = Float::to_bits(x);
    /// let y: f32 = Float::from_bits(bits);
    /// assert_eq!(y, x);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_bits(bits: Self::Bits) -> Self;

    /// Raw transmutation from a floating-point type to an unsigned integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Float;
    /// let x: f32 = 2.5;
    /// let bits: u32 = Float::to_bits(x);
    /// let y: f32 = Float::from_bits(bits);
    /// assert_eq!(y, x);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_bits(self) -> Self::Bits;

    /// Converts a signed integer to the nearest representable floating-point value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Float;
    /// let x: f32 = Float::from_int(-3);
    /// assert_eq!(x, -3.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_int(int: <Self::Bits as Int>::Signed) -> Self;

    /// Rounds `self` toward zero and converts it to a signed integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Float;
    /// let x: f32 = 3.7;
    /// let y: i32 = Float::to_int(x);
    /// assert_eq!(y, 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_int(self) -> <Self::Bits as Int>::Signed;
}

macro_rules! impl_float {
    ($($float:ty => $uint:ty, $int:ty),*) => {
        $(
            impl Float for $float {
                type Bits = $uint;

                const RADIX: u32 = <$float>::RADIX;
                const MANTISSA_DIGITS: u32 = <$float>::MANTISSA_DIGITS;
                const DIGITS: u32 = <$float>::DIGITS;
                const EPSILON: Self = <$float>::EPSILON;
                const MIN_POSITIVE: Self = <$float>::MIN_POSITIVE;
                const MIN_EXP: i32 = <$float>::MIN_EXP;
                const MAX_EXP: i32 = <$float>::MAX_EXP;
                const MIN_10_EXP: i32 = <$float>::MIN_10_EXP;
                const MAX_10_EXP: i32 = <$float>::MAX_10_EXP;
                const NAN: Self = <$float>::NAN;
                const INFINITY: Self = <$float>::INFINITY;
                const NEG_INFINITY: Self = <$float>::NEG_INFINITY;

                #[inline]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    self.mul_add(a, b)
                }
                #[inline]
                fn powf(self, n: Self) -> Self {
                    self.powf(n)
                }
                #[inline]
                fn is_nan(self) -> bool {
                    self.is_nan()
                }
                #[inline]
                fn is_infinite(self) -> bool {
                    self.is_infinite()
                }
                #[inline]
                fn is_finite(self) -> bool {
                    self.is_finite()
                }
                #[inline]
                fn is_subnormal(self) -> bool {
                    self.is_subnormal()
                }
                #[inline]
                fn is_normal(self) -> bool {
                    self.is_normal()
                }
                #[inline]
                fn is_sign_positive(self) -> bool {
                    self.is_sign_positive()
                }
                #[inline]
                fn is_sign_negative(self) -> bool {
                    self.is_sign_negative()
                }
                #[inline]
                fn next_up(self) -> Self {
                    self.next_up()
                }
                #[inline]
                fn next_down(self) -> Self {
                    self.next_down()
                }
                #[inline]
                fn from_bits(bits: Self::Bits) -> Self {
                    <$float>::from_bits(bits)
                }
                #[inline]
                fn to_bits(self) -> Self::Bits {
                    <$float>::to_bits(self)
                }
                #[inline]
                fn from_int(int: $int) -> Self {
                    int as $float
                }
                #[inline]
                fn to_int(self) -> $int {
                    self as $int
                }
            }
        )*
    };
}

impl_float!(f32 => u32, i32, f64 => u64, i64);

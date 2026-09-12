use super::NumEq;
use crate::simd::Select;

/// Element-wise ordering comparisons for numeric types.
///
/// This is implemented for both scalar and SIMD types, so that
/// the same interface can be used for both. For scalar types,
/// the mask type is always [`bool`].
///
/// Unlike [`Ord`], this trait does not require a total ordering,
/// so it can be implemented for floating-point types as well.
/// However, unlike [`PartialOrd`], this trait still has methods
/// for computing the minimum and maximum of two values,
/// and for clamping a value to an interval.
///
/// # Floating-Point Types
///
/// For floating-point types, the [`min`], [`max`], and [`clamp`] methods follow
/// the semantics of [`f32::min`], [`f32::max`], and [`f32::clamp`] respectively.
///
/// Additional methods [`min_fast`](Self::min_fast), [`max_fast`](Self::max_fast),
/// and [`clamp_fast`](Self::clamp_fast) are provided as faster and fully deterministic
/// alternatives, but they differ from the standard methods in how they handle NaN and signed zero.
/// See the documentation of each method for more information.
///
/// [`min`]: Self::min
/// [`max`]: Self::max
/// [`clamp`]: Self::clamp
/// [`min_fast`]: Self::min_fast
/// [`max_fast`]: Self::max_fast
/// [`clamp_fast`]: Self::clamp_fast
pub trait NumOrd: NumEq {
    /// Test if each element is less than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn num_lt(self, other: Self) -> Self::Bool;

    /// Test if each element is less than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn num_le(self, other: Self) -> Self::Bool;

    /// Test if each element is greater than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn num_gt(self, other: Self) -> Self::Bool;

    /// Test if each element is greater than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn num_ge(self, other: Self) -> Self::Bool;

    /// Returns the element-wise minimum with `other`.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method follows the semantics of [`f32::min`].
    ///
    /// - If exactly one of the arguments is NaN (quiet or signaling), then the other argument is returned.
    /// - If both arguments are NaN, the return value is NaN, with the bit pattern picked using the usual rules
    ///   for arithmetic operations.
    /// - If the inputs compare equal (such as for the case of `+0.0` and `-0.0`), either input may be returned non-deterministically.
    ///
    /// See [`min_fast`](Self::min_fast) for a version that handles NaN and signed zero differently
    /// but is faster on some platforms and is fully deterministic.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn min(self, other: Self) -> Self;

    /// Returns the element-wise maximum with `other`.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method follows the semantics of [`f32::max`].
    ///
    /// - If exactly one of the arguments is NaN (quiet or signaling), then the other argument is returned.
    /// - If both arguments are NaN, the return value is NaN, with the bit pattern picked using the usual rules for arithmetic operations.
    /// - If the inputs compare equal (such as for the case of `+0.0` and `-0.0`), either input may be returned non-deterministically.
    ///
    /// See [`max_fast`](Self::max_fast) for a version that handles NaN and signed zero differently
    /// but is faster on some platforms and is fully deterministic.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn max(self, other: Self) -> Self;

    /// Restricts each element to a certain interval.
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self`
    /// is less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN on any element.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method follows the semantics of [`f32::clamp`].
    ///
    /// - If the initial value is NaN, the result is NaN.
    /// - If the result is zero and among the three inputs `self`, `min`, and `max` there are zeros
    ///   with different sign, either `+0.0` or `-0.0` is returned non-deterministically.
    ///
    /// See [`clamp_fast`](Self::clamp_fast) for a version that handles NaN and signed zero differently
    /// but is faster on some platforms and is fully deterministic.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Returns the element-wise minimum with `other`, using the comparison `if self < other`.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method differs from [`min`](Self::min) in that it does not
    /// handle NaN or signed zero specially. It _always_ returns `other` if `self` does not compare
    /// less than `other`, even if either value is NaN or if the two values compare equal
    /// (such as for the case of `+0.0` and `-0.0`).
    ///
    /// The method is equivalent to simply using [`num_lt`](Self::num_lt) and [`select`](crate::simd::Select::select)
    /// to choose between the two values. This is faster than [`min`](Self::min) on all tested platforms,
    /// and also has the benefit of being fully deterministic.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn min_fast(self, other: Self) -> Self
    where
        Self: Copy,
    {
        self.num_lt(other).select(self, other)
    }

    /// Returns the element-wise maximum with `other`, using the comparison `if self > other`.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method differs from [`max`](Self::max) in that it does not
    /// handle NaN or signed zero specially. It _always_ returns `other` if `self` does not compare
    /// greater than `other`, even if either value is NaN or if the two values compare equal
    /// (such as for the case of `+0.0` and `-0.0`).
    ///
    /// The method is equivalent to simply using [`num_gt`](Self::num_gt) and [`select`](crate::simd::Select::select)
    /// to choose between the two values. This is faster than [`max`](Self::max) on all tested platforms,
    /// and also has the benefit of being fully deterministic.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn max_fast(self, other: Self) -> Self
    where
        Self: Copy,
    {
        self.num_gt(other).select(self, other)
    }

    /// Restricts each element to an interval, using [`min_fast`](Self::min_fast)
    /// and [`max_fast`](Self::max_fast).
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self`
    /// is less than `min`. Otherwise returns `self`.
    ///
    /// Unlike [`clamp`](Self::clamp), this does not panic if `min > max`. The bounds
    /// are applied in order, so if they are reversed, the result is always `max`.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, this method differs from [`clamp`](Self::clamp) in that it does not
    /// handle NaN or signed zero specially. It _always_ returns `min` if `self` does not compare
    /// greater than `min`, and otherwise it _always_ returns `max` if `self` does not compare less
    /// than `max`. This causes some non-intuitive behavior when NaN or signed zero is involved:
    ///
    /// ```
    /// use core::f32::NAN;
    /// use gnum::cmp::NumOrd;
    ///
    /// // NaN input is not propagated, instead the bounds are applied in order.
    /// assert_eq!(NumOrd::clamp_fast(NAN, 0.0, 1.0), 0.0);
    ///
    /// // NaN as a lower bound forces the result to be the upper bound.
    /// assert_eq!(NumOrd::clamp_fast(0.5, NAN, 1.0), 1.0);
    ///
    /// // NaN as an upper bound leaks through as NaN.
    /// assert!(NumOrd::clamp_fast(0.5, 0.0, NAN).is_nan());
    ///
    /// // Signed zero is not preserved, instead the bounds are applied in order.
    /// assert_eq!(NumOrd::clamp_fast(-0.0f32, 0.0, 1.0), 0.0);
    /// ```
    ///
    /// The method is equivalent to simply using [`max_fast`](Self::max_fast) and [`min_fast`](Self::min_fast)
    /// to clamp the value. This is faster than [`clamp`](Self::clamp) on all tested platforms,
    /// and also has the benefit of being fully deterministic.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn clamp_fast(self, min: Self, max: Self) -> Self
    where
        Self: Copy,
    {
        self.max_fast(min).min_fast(max)
    }
}

macro_rules! impl_num_ord_scalar_int {
    ($($int:ty),*) => {
        $(
            impl NumOrd for $int {
                #[inline]
                fn num_lt(self, other: Self) -> Self::Bool {
                    self < other
                }

                #[inline]
                fn num_le(self, other: Self) -> Self::Bool {
                    self <= other
                }

                #[inline]
                fn num_gt(self, other: Self) -> Self::Bool {
                    self > other
                }

                #[inline]
                fn num_ge(self, other: Self) -> Self::Bool {
                    self >= other
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    Ord::min(self, other)
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    Ord::max(self, other)
                }

                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    Ord::clamp(self, min, max)
                }
            }
        )*
    };
}

impl_num_ord_scalar_int!(u8, u16, u32, u64, usize);
impl_num_ord_scalar_int!(i8, i16, i32, i64, isize);

macro_rules! impl_num_ord_scalar_float {
    ($($float:ty),*) => {
        $(
            impl NumOrd for $float {
                #[inline]
                fn num_lt(self, other: Self) -> Self::Bool {
                    self < other
                }

                #[inline]
                fn num_le(self, other: Self) -> Self::Bool {
                    self <= other
                }

                #[inline]
                fn num_gt(self, other: Self) -> Self::Bool {
                    self > other
                }

                #[inline]
                fn num_ge(self, other: Self) -> Self::Bool {
                    self >= other
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    self.min(other)
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    self.max(other)
                }

                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }
            }
        )*
    };
}

impl_num_ord_scalar_float!(f32, f64);

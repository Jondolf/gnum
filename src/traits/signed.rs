use crate::{simd::SimdValue, traits::Num};

/// A trait for signed types such as [`i32`] and [`f32`].
pub trait Signed: Num + SimdValue {
    /// Returns the absolute value of `self`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs(self) -> Self;

    /// Returns the positive difference between `self` and `rhs`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs_diff(self, rhs: Self) -> Self;

    /// Returns the sign of `self`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn signum(self) -> Self;

    /// Returns a number with the magnitude of `self` and the sign of `sign`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn copysign(self, sign: Self) -> Self;

    /// Returns `true` if `self` is positive, and `false` if the integer is zero or negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_positive(self) -> Self::Bool;

    /// Returns `true` if `self` is negative, and `false` if the integer is zero or positive.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_negative(self) -> Self::Bool;
}

use crate::{
    simd::SimdValue,
    traits::{NegOne, Num},
};
use core::ops::Neg;

/// A trait for signed types such as [`i32`] and [`f32`].
pub trait Signed: Num + NegOne + SimdValue + Neg<Output = Self> {
    /// The corresponding unsigned type for this signed type.
    type Unsigned;

    /// Returns the absolute value of `self`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs(self) -> Self;

    /// Returns the positive difference between `self` and `rhs`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs_diff(self, rhs: Self) -> Self::Unsigned;

    /// Returns the sign of `self`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn signum(self) -> Self;

    /// Returns `true` if `self` is positive, and `false` if the integer is zero or negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_positive(self) -> Self::Bool;

    /// Returns `true` if `self` is negative, and `false` if the integer is zero or positive.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_negative(self) -> Self::Bool;
}

macro_rules! impl_signed_int {
    ($($int:ty => $uint:ty),*) => {
        $(
            impl Signed for $int {
                type Unsigned = $uint;

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }
                #[inline]
                fn abs_diff(self, rhs: Self) -> Self::Unsigned {
                    self.abs_diff(rhs)
                }
                #[inline]
                fn signum(self) -> Self {
                    self.signum()
                }
                #[inline]
                fn is_positive(self) -> Self::Bool {
                    self.is_positive()
                }
                #[inline]
                fn is_negative(self) -> Self::Bool {
                    self.is_negative()
                }
            }
        )*
    };
}

impl_signed_int!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

macro_rules! impl_signed_float {
    ($($float:ty),*) => {
        $(
            impl Signed for $float {
                type Unsigned = Self;

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }
                #[inline]
                fn abs_diff(self, rhs: Self) -> Self {
                    (self - rhs).abs()
                }
                #[inline]
                fn signum(self) -> Self {
                    self.signum()
                }
                #[inline]
                fn is_positive(self) -> Self::Bool {
                    self > 0.0
                }
                #[inline]
                fn is_negative(self) -> Self::Bool {
                    self < 0.0
                }
            }
        )*
    };
}

impl_signed_float!(f32, f64);

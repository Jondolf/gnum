use core::ops::*;

use crate::num::ops::{impl_binary_op, impl_unary_op};

/// Performs saturating addition that saturates at the numeric bounds instead of overflowing.
///
/// This is a saturating version of the [`Add`] trait.
pub trait SaturatingAdd<Rhs = Self>: Add<Rhs> {
    /// Performs the `+` operation, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_add(1), 13);
    /// assert_eq!(i32::MAX.saturating_add(1), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

/// Performs saturating subtraction that saturates at the numeric bounds instead of overflowing.
///
/// This is a saturating version of the [`Sub`] trait.
pub trait SaturatingSub<Rhs = Self>: Sub<Rhs> {
    /// Performs the `-` operation, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_sub(1), 11);
    /// assert_eq!(i32::MIN.saturating_sub(1), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

/// Performs saturating multiplication that saturates at the numeric bounds instead of overflowing.
///
/// This is a saturating version of the [`Mul`] trait.
pub trait SaturatingMul<Rhs = Self>: Mul<Rhs> {
    /// Performs the `*` operation, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_mul(2), 24);
    /// assert_eq!(i32::MAX.saturating_mul(2), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

/// Performs saturating division that saturates at the numeric bounds instead of overflowing.
///
/// This is a saturating version of the [`Div`] trait.
pub trait SaturatingDiv<Rhs = Self>: Div<Rhs> {
    /// Performs the `/` operation, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_div(2), 6);
    /// assert_eq!(12.saturating_div(0), i32::MAX); // or i32::MIN for signed types
    /// assert_eq!(i32::MIN.saturating_div(-1), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}

/// Performs saturating negation that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingNeg: Neg {
    /// Performs the unary `-` operation, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_neg(), -12);
    /// assert_eq!(i32::MIN.saturating_neg(), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_neg(self) -> Self::Output;
}

/// Performs saturating absolute value calculation that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingAbs: Sized {
    /// The resulting type after applying the saturating absolute value operation.
    type Output;

    /// Returns the absolute value of `self`, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.saturating_abs(), 12);
    /// assert_eq!((-12).saturating_abs(), 12);
    /// assert_eq!(i32::MIN.saturating_abs(), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_abs(self) -> Self::Output;
}

/// Performs saturating exponentiation that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingPow<Exp = Self>: Sized {
    /// The resulting type after applying the saturating exponentiation operation.
    type Output;

    /// Returns `self` raised to the power of `exp`, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.saturating_pow(3), 8);
    /// assert_eq!(i32::MAX.saturating_pow(2), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_pow(self, exp: Exp) -> Self::Output;
}

impl_binary_op!(
    SaturatingAdd,
    saturating_add,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

impl_binary_op!(
    SaturatingSub,
    saturating_sub,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

impl_binary_op!(
    SaturatingMul,
    saturating_mul,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

impl_binary_op!(
    SaturatingDiv,
    saturating_div,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

impl_unary_op!(
    SaturatingNeg,
    saturating_neg,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

macro_rules! impl_saturating_abs {
    ($($t:ty),*) => {
        $(
            impl SaturatingAbs for $t {
                type Output = $t;

                #[inline]
                fn saturating_abs(self) -> Self::Output {
                    <$t>::saturating_abs(self)
                }
            }

            impl SaturatingAbs for &$t {
                type Output = $t;

                #[inline]
                fn saturating_abs(self) -> Self::Output {
                    <$t>::saturating_abs(*self)
                }
            }
        )*
    };
}

impl_saturating_abs!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_saturating_pow {
    ($($t:ty),*) => {
        $(
            impl SaturatingPow<u32> for $t {
                type Output = $t;

                #[inline]
                fn saturating_pow(self, exp: u32) -> Self::Output {
                    <$t>::saturating_pow(self, exp)
                }
            }

            impl SaturatingPow<u32> for &$t {
                type Output = $t;

                #[inline]
                fn saturating_pow(self, exp: u32) -> Self::Output {
                    <$t>::saturating_pow(*self, exp)
                }
            }
        )*
    };
}

impl_saturating_pow!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

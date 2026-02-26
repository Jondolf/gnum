use core::ops::*;

/// Performs wrapping addition that wraps around on overflow.
///
/// This is a wrapping version of the [`Add`] trait.
pub trait WrappingAdd<Rhs = Self>: Add<Rhs> {
    /// Performs the `+` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_add(1), 13);
    /// assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping subtraction that wraps around on overflow.
///
/// This is a wrapping version of the [`Sub`] trait.
pub trait WrappingSub<Rhs = Self>: Sub<Rhs> {
    /// Performs the `-` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_sub(1), 11);
    /// assert_eq!(i32::MIN.wrapping_sub(1), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping multiplication that wraps around on overflow.
///
/// This is a wrapping version of the [`Mul`] trait.
pub trait WrappingMul<Rhs = Self>: Mul<Rhs> {
    /// Performs the `*` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_mul(2), 24);
    /// assert_eq!(i32::MAX.wrapping_mul(2), -2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping division that wraps around on overflow.
///
/// This is a wrapping version of the [`Div`] trait.
pub trait WrappingDiv<Rhs = Self>: Div<Rhs> {
    /// Performs the `/` operation, wrapping around on overflow.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_div(2), 6);
    /// assert_eq!(i32::MIN.wrapping_div(-1), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping remainder that wraps around on overflow.
///
/// This is a wrapping version of the [`Rem`] trait.
pub trait WrappingRem<Rhs = Self>: Rem<Rhs> {
    /// Performs the `%` operation, wrapping around on overflow.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_rem(5), 2);
    /// assert_eq!(i32::MIN.wrapping_rem(-1), 0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping Euclidean division that wraps around on overflow.
pub trait WrappingDivEuclid<Rhs = Self>: Div<Rhs> {
    /// Performs the `div_euclid` operation, wrapping around on overflow.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_div_euclid(5), 2);
    /// assert_eq!(i32::MIN.wrapping_div_euclid(-1), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_div_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping Euclidean remainder that wraps around on overflow.
pub trait WrappingRemEuclid<Rhs = Self>: Rem<Rhs> {
    /// Performs the `rem_euclid` operation, wrapping around on overflow.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_rem_euclid(5), 2);
    /// assert_eq!(i32::MIN.wrapping_rem_euclid(-1), 0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_rem_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping negation that wraps around on overflow.
///
/// This is a wrapping version of the [`Neg`] trait.
pub trait WrappingNeg: Neg {
    /// Performs the unary `-` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_neg(), -12);
    /// assert_eq!(i32::MIN.wrapping_neg(), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_neg(self) -> Self::Output;
}

/// Performs wrapping left bitwise shift that wraps around on overflow.
///
/// This is a wrapping version of the [`Shl`] trait.
pub trait WrappingShl<Rhs = Self>: Shl<Rhs> {
    /// Performs the `<<` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(1u32.wrapping_shl(2), 4);
    /// assert_eq!(1u32.wrapping_shl(32), 1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shl(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping right bitwise shift that wraps around on overflow.
///
/// This is a wrapping version of the [`Shr`] trait.
pub trait WrappingShr<Rhs = Self>: Shr<Rhs> {
    /// Performs the `>>` operation, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(4u32.wrapping_shr(2), 1);
    /// assert_eq!(4u32.wrapping_shr(32), 4);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_shr(self, rhs: Rhs) -> Self::Output;
}

/// Performs wrapping absolute value calculation that wraps around on overflow.
pub trait WrappingAbs: Sized {
    /// Returns the absolute value of `self`, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.wrapping_abs(), 12);
    /// assert_eq!((-12).wrapping_abs(), 12);
    /// assert_eq!(i32::MIN.wrapping_abs(), i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_abs(self) -> Self;
}

/// Performs wrapping exponentiation that wraps around on overflow.
pub trait WrappingPow<Exp = Self>: Sized {
    /// Returns `self` raised to the power of `exp`, wrapping around on overflow.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.wrapping_pow(3), 8);
    /// assert_eq!(i32::MAX.wrapping_pow(2), 1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_pow(self, exp: Exp) -> Self;
}

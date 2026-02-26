use core::ops::*;

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

/// Performs saturating exponentiation that saturates at the numeric bounds instead of overflowing.
pub trait SaturatingPow<Exp = Self>: Sized {
    /// Returns `self` raised to the power of `exp`, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.saturating_pow(3), 8);
    /// assert_eq!(i32::MAX.saturating_pow(2), i32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_pow(self, exp: Exp) -> Self;
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
    fn saturating_abs(self) -> Self;
}

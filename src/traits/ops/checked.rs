use core::ops::*;

/// Performs addition that returns `None` instead of wrapping around on overflow.
///
/// This is a checked version of the [`Add`] trait.
pub trait CheckedAdd<Rhs = Self>: Add<Rhs> + Sized {
    /// Performs the `+` operation, returning `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_add(1), Some(13));
    /// assert_eq!(i32::MAX.checked_add(1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `+` operation, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when performing the addition.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_add(1), 13);
    /// ``````
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MAX.strict_add(1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_add(self, rhs: Rhs) -> Self::Output;

    /// Performs the `+` operation, without checking for overflow.
    ///
    /// Calling `x.unchecked_add(y)` is semantically equivalent to calling [`checked_add`](Self::checked_add)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if overflow occurs when performing the addition.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_add(self, rhs: Rhs) -> Self::Output;
}

/// Performs subtraction that returns `None` instead of wrapping around on overflow.
///
/// This is a checked version of the [`Sub`] trait.
pub trait CheckedSub<Rhs = Self>: Sub<Rhs> + Sized {
    /// Performs the `-` operation, returning `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_sub(1), Some(11));
    /// assert_eq!(i32::MIN.checked_sub(1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `-` operation, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when performing the subtraction.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_sub(1), 11);
    /// ``````
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_sub(1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_sub(self, rhs: Rhs) -> Self::Output;

    /// Performs the `-` operation, without checking for overflow.
    ///
    /// Calling `x.unchecked_sub(y)` is semantically equivalent to calling [`checked_sub`](Self::checked_sub)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if overflow occurs when performing the subtraction.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_sub(self, rhs: Rhs) -> Self::Output;
}

/// Performs multiplication that returns `None` instead of wrapping around on overflow.
///
/// This is a checked version of the [`Mul`] trait.
pub trait CheckedMul<Rhs = Self>: Mul<Rhs> + Sized {
    /// Performs the `*` operation, returning `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_mul(2), Some(24));
    /// assert_eq!(i32::MAX.checked_mul(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `*` operation, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when performing the multiplication.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_mul(2), 24);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MAX.strict_mul(2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_mul(self, rhs: Rhs) -> Self::Output;

    /// Performs the `*` operation, without checking for overflow.
    ///
    /// Calling `x.unchecked_mul(y)` is semantically equivalent to calling [`checked_mul`](Self::checked_mul)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if overflow occurs when performing the multiplication.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_mul(self, rhs: Rhs) -> Self::Output;
}

/// Performs division that returns `None` instead of wrapping around on overflow or division by zero.
///
/// This is a checked version of the [`Div`] trait.
pub trait CheckedDiv<Rhs = Self>: Div<Rhs> + Sized {
    /// Performs the `/` operation, returning `None` if overflow or division by zero occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_div(2), Some(6));
    /// assert_eq!(12.checked_div(0), None);
    /// assert_eq!(i32::MIN.checked_div(-1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `/` operation, panicking if overflow or division by zero occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow or division by zero occurs when performing the division.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_div(2), 6);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// let _ = 12.strict_div(0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_div(-1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_div(self, rhs: Rhs) -> Self::Output;
}

/// Performs remainder that returns `None` instead of wrapping around on overflow or division by zero.
///
/// This is a checked version of the [`Rem`] trait.
pub trait CheckedRem<Rhs = Self>: Rem<Rhs> + Sized {
    /// Performs the `%` operation, returning `None` if overflow or division by zero occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_rem(5), Some(2));
    /// assert_eq!(12.checked_rem(0), None);
    /// assert_eq!(i32::MIN.checked_rem(-1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `%` operation, panicking if overflow or division by zero occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow or division by zero occurs when performing the remainder operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_rem(5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_rem(self, rhs: Rhs) -> Self::Output;
}

/// Performs Euclidean division that returns `None` instead of wrapping around on overflow or division by zero.
pub trait CheckedDivEuclid<Rhs = Self>: Div<Rhs> + Sized {
    /// Performs the `div_euclid` operation, returning `None` if overflow or division by zero occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_div_euclid(5), Some(2));
    /// assert_eq!(12.checked_div_euclid(0), None);
    /// assert_eq!(i32::MIN.checked_div_euclid(-1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div_euclid(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `div_euclid` operation, panicking if overflow or division by zero occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow or division by zero occurs when performing the Euclidean division.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_div_euclid(5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// let _ = 12.strict_div_euclid(0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_div_euclid(-1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_div_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Performs Euclidean remainder that returns `None` instead of wrapping around on overflow or division by zero.
pub trait CheckedRemEuclid<Rhs = Self>: Rem<Rhs> + Sized {
    /// Performs the `rem_euclid` operation, returning `None` if overflow or division by zero occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_rem_euclid(5), Some(2));
    /// assert_eq!(12.checked_rem_euclid(0), None);
    /// assert_eq!(i32::MIN.checked_rem_euclid(-1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_rem_euclid(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `rem_euclid` operation, panicking if overflow or division by zero occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow or division by zero occurs when performing the Euclidean remainder operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_rem_euclid(5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// let _ = 12.strict_rem_euclid(0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_rem_euclid(-1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_rem_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Performs exact division that returns `None` instead of wrapping around on overflow, division by zero, or if the division is not exact.
pub trait CheckedDivExact<Rhs = Self>: Div<Rhs> + Sized {
    /// Performs the `div_exact` operation, returning `None` if overflow, division by zero, or inexact division occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_div_exact(3), Some(4));
    /// assert_eq!(12.checked_div_exact(5), None);
    /// assert_eq!(12.checked_div_exact(0), None);
    /// assert_eq!(i32::MIN.checked_div_exact(-1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_div_exact(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `div_exact` operation, panicking if overflow, division by zero, or inexact division occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow, division by zero, or inexact division occurs when performing the exact division.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_div_exact(3), Some(4));
    /// assert_eq!(12.strict_div_exact(5), None);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// let _ = 12.strict_div_exact(0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_div_exact(-1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_div_exact(self, rhs: Rhs) -> Self::Output;
}

/// Performs negation that returns `None` instead of wrapping around on overflow.
///
/// This is a checked version of the [`Neg`] trait.
pub trait CheckedNeg: Neg<Output = Self> + Sized {
    /// Performs the unary `-` operation, returning `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_neg(), Some(-12));
    /// assert_eq!(i32::MIN.checked_neg(), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_neg(self) -> Option<Self::Output>;

    /// Performs the unary `-` operation, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when performing the negation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_neg(), -12);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_neg();
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_neg(self) -> Self::Output;

    /// Performs the unary `-` operation, without checking for overflow.
    ///
    /// Calling `x.unchecked_neg()` is semantically equivalent to calling [`checked_neg`](Self::checked_neg)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if overflow occurs when performing the negation.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_neg(self) -> Self::Output;
}

/// Performs left bitwise shift that returns `None` if the shift amount is greater than
/// or equal to the number of bits in the type.
///
/// This is a checked version of the [`Shl`] trait.
pub trait CheckedShl<Rhs = Self>: Shl<Rhs> + Sized {
    /// Performs the `<<` operation, returning `None` if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(1u32.checked_shl(2), Some(4));
    /// assert_eq!(1u32.checked_shl(32), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shl(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `<<` operation, panicking if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// # Panics
    ///
    /// Panics if the shift amount is greater than or equal to the number of bits in the type.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(1u32.strict_shl(2), 4);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// let _ = 1u32.strict_shl(32);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shl(self, rhs: Rhs) -> Self::Output;

    /// Performs the `<<` operation, without checking if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// Calling `x.unchecked_shl(y)` is semantically equivalent to calling [`checked_shl`](Self::checked_shl)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if the shift amount is greater than or equal to the number of bits in the type.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_shl(self, rhs: Rhs) -> Self::Output;
}

/// Performs exact left bitwise shift that returns `None` if the shift amount is greater than
/// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
pub trait CheckedShlExact<Rhs = Self>: Shl<Rhs> + Sized {
    /// Performs the `<<` operation, returning `None` if the shift amount is greater than
    /// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(1u32.checked_shl_exact(2), Some(4));
    /// assert_eq!(1u32.checked_shl_exact(32), None);
    /// assert_eq!(3u32.checked_shl_exact(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn shl_exact(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `<<` operation, panicking if the shift amount is greater than
    /// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Panics
    ///
    /// Panics if the shift amount is greater than or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(1u32.strict_shl_exact(2), 4);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// let _ = 1u32.strict_shl_exact(32);
    /// ```
    ///
    /// The following panics because some of the bits shifted out are not zero:
    ///
    /// ```should_panic
    /// let _ = 3u32.strict_shl_exact(2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shl_exact(self, rhs: Rhs) -> Self::Output;
}

/// Performs right bitwise shift that returns `None` if the shift amount is greater than
/// or equal to the number of bits in the type.
///
/// This is a checked version of the [`Shr`] trait.
pub trait CheckedShr<Rhs = Self>: Shr<Rhs> + Sized {
    /// Performs the `>>` operation, returning `None` if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(4u32.checked_shr(2), Some(1));
    /// assert_eq!(4u32.checked_shr(32), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_shr(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `>>` operation, panicking if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// # Panics
    ///
    /// Panics if the shift amount is greater than or equal to the number of bits in the type.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(4u32.strict_shr(2), 1);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// let _ = 4u32.strict_shr(32);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shr(self, rhs: Rhs) -> Self::Output;

    /// Performs the `>>` operation, without checking if the shift amount is greater than
    /// or equal to the number of bits in the type.
    ///
    /// Calling `x.unchecked_shr(y)` is semantically equivalent to calling [`checked_shr`](Self::checked_shr)
    /// and [`unwrap_unchecked`](Option::unwrap_unchecked).
    ///
    /// # Safety
    ///
    /// This results in undefined behavior if the shift amount is greater than or equal to the number of bits in the type.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    unsafe fn unchecked_shr(self, rhs: Rhs) -> Self::Output;
}

/// Performs exact right bitwise shift that returns `None` if the shift amount is greater than
/// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
pub trait CheckedShrExact<Rhs = Self>: Shr<Rhs> + Sized {
    /// Performs the `>>` operation, returning `None` if the shift amount is greater than
    /// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(4u32.checked_shr_exact(2), Some(1));
    /// assert_eq!(4u32.checked_shr_exact(32), None);
    /// assert_eq!(5u32.checked_shr_exact(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn shr_exact(self, rhs: Rhs) -> Option<Self::Output>;

    /// Performs the `>>` operation, panicking if the shift amount is greater than
    /// or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Panics
    ///
    /// Panics if the shift amount is greater than or equal to the number of bits in the type, or if any of the bits shifted out are not zero.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(4u32.strict_shr_exact(2), 1);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// let _ = 4u32.strict_shr_exact(32);
    /// ```
    ///
    /// The following panics because some of the bits shifted out are not zero:
    ///
    /// ```should_panic
    /// let _ = 5u32.strict_shr_exact(2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_shr_exact(self, rhs: Rhs) -> Self::Output;
}

/// Performs checked absolute value calculation that returns `None` instead of wrapping around on overflow.
pub trait CheckedAbs: Sized {
    /// Returns the absolute value of `self`, or `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.checked_abs(), Some(12));
    /// assert_eq!((-12).checked_abs(), Some(12));
    /// assert_eq!(i32::MIN.checked_abs(), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_abs(self) -> Option<Self>;

    /// Returns the absolute value of `self`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when calculating the absolute value.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12.strict_abs(), 12);
    /// assert_eq!((-12).strict_abs(), 12);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MIN.strict_abs();
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_abs(self) -> Self;
}

/// Performs checked exponentiation that returns `None` instead of wrapping around on overflow.
pub trait CheckedPow<Exp = Self>: Sized {
    /// Returns `self` raised to the power of `exp`, or `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.checked_pow(3), Some(8));
    /// assert_eq!(i32::MAX.checked_pow(2), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_pow(self, exp: Exp) -> Option<Self>;

    /// Returns `self` raised to the power of `exp`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when calculating the exponentiation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.strict_pow(3), 8);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// let _ = i32::MAX.strict_pow(2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_pow(self, exp: Exp) -> Self;
}

/// Performs checked integer square root calculation that returns `None` if `self` is negative.
pub trait CheckedISqrt: Sized {
    /// Returns the integer square root of `self`, rounded down, or `None` if `self` is negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.checked_isqrt(), Some(4));
    /// assert_eq!(15.checked_isqrt(), Some(3));
    /// assert_eq!((-1).checked_isqrt(), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_isqrt(self) -> Option<Self>;
}

/// Performs checked integer logarithm calculation that returns `None` if `self` is zero or negative,
/// or if `base` is less than 2.
pub trait CheckedILog: Sized {
    /// Returns the integer logarithm of `self` with respect to `base`, rounded down,
    /// or `None` if `self` is zero or negative, or if `base` is less than 2.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.checked_ilog(2), Some(4));
    /// assert_eq!(15.checked_ilog(2), Some(3));
    /// assert_eq!(0.checked_ilog(2), None);
    /// assert_eq!((-1).checked_ilog(2), None);
    /// assert_eq!(16.checked_ilog(1), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog(self, base: Self) -> Option<Self>;

    /// Returns the base 2 integer logarithm of `self`, rounded down,
    /// or `None` if `self` is zero or negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.checked_ilog2(), Some(4));
    /// assert_eq!(15.checked_ilog2(), Some(3));
    /// assert_eq!(0.checked_ilog2(), None);
    /// assert_eq!((-1).checked_ilog2(), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog2(self) -> Option<Self>;

    /// Returns the base 10 integer logarithm of `self`, rounded down,
    /// or `None` if `self` is zero or negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(100.checked_ilog10(), Some(2));
    /// assert_eq!(99.checked_ilog10(), Some(1));
    /// assert_eq!(0.checked_ilog10(), None);
    /// assert_eq!((-1).checked_ilog10(), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_ilog10(self) -> Option<Self>;
}

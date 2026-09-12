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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedAdd::checked_add(12, 1), Some(13));
    /// assert_eq!(CheckedAdd::checked_add(i32::MAX, 1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedAdd::strict_add(12, 1), 13);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedAdd::strict_add(i32::MAX, 1);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedSub::checked_sub(12, 1), Some(11));
    /// assert_eq!(CheckedSub::checked_sub(i32::MIN, 1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedSub::strict_sub(12, 1), 11);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedSub::strict_sub(i32::MIN, 1);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedMul::checked_mul(12, 2), Some(24));
    /// assert_eq!(CheckedMul::checked_mul(i32::MAX, 2), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedMul::strict_mul(12, 2), 24);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedMul::strict_mul(i32::MAX, 2);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedDiv::checked_div(12, 2), Some(6));
    /// assert_eq!(CheckedDiv::checked_div(12, 0), None);
    /// assert_eq!(CheckedDiv::checked_div(i32::MIN, -1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedDiv::strict_div(12, 2), 6);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedDiv::strict_div(12, 0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedDiv::strict_div(i32::MIN, -1);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedRem::checked_rem(12, 5), Some(2));
    /// assert_eq!(CheckedRem::checked_rem(12, 0), None);
    /// assert_eq!(CheckedRem::checked_rem(i32::MIN, -1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedRem::strict_rem(12, 5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedRem::strict_rem(12, 0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedRem::strict_rem(i32::MIN, -1);
    /// ```
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(12, 5), Some(2));
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(12, 0), None);
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(i32::MIN, -1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedDivEuclid::strict_div_euclid(12, 5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedDivEuclid::strict_div_euclid(12, 0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedDivEuclid::strict_div_euclid(i32::MIN, -1);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedRemEuclid::checked_rem_euclid(12, 5), Some(2));
    /// assert_eq!(CheckedRemEuclid::checked_rem_euclid(12, 0), None);
    /// assert_eq!(CheckedRemEuclid::checked_rem_euclid(i32::MIN, -1), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedRemEuclid::strict_rem_euclid(12, 5), 2);
    /// ```
    ///
    /// The following panics because of division by zero:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedRemEuclid::strict_rem_euclid(12, 0);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedRemEuclid::strict_rem_euclid(i32::MIN, -1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_rem_euclid(self, rhs: Rhs) -> Self::Output;
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedNeg::checked_neg(12), Some(-12));
    /// assert_eq!(CheckedNeg::checked_neg(i32::MIN), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedNeg::strict_neg(12), -12);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedNeg::strict_neg(i32::MIN);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedShl::checked_shl(1u32, 2), Some(4));
    /// assert_eq!(CheckedShl::checked_shl(1u32, 32), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedShl::strict_shl(1u32, 2), 4);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedShl::strict_shl(1u32, 32);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedShr::checked_shr(4u32, 2), Some(1));
    /// assert_eq!(CheckedShr::checked_shr(4u32, 32), None);
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
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedShr::strict_shr(4u32, 2), 1);
    /// ```
    ///
    /// The following panics because the shift amount is greater than or equal to the number of bits in the type:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedShr::strict_shr(4u32, 32);
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

/// Performs checked absolute value calculation that returns `None` instead of wrapping around on overflow.
pub trait CheckedAbs: Sized {
    /// The resulting type after applying the absolute value operation.
    type Output;

    /// Returns the absolute value of `self`, or `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedAbs::checked_abs(12), Some(12));
    /// assert_eq!(CheckedAbs::checked_abs(-12), Some(12));
    /// assert_eq!(CheckedAbs::checked_abs(i32::MIN), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_abs(self) -> Option<Self::Output>;

    /// Returns the absolute value of `self`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when calculating the absolute value.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedAbs::strict_abs(12), 12);
    /// assert_eq!(CheckedAbs::strict_abs(-12), 12);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedAbs::strict_abs(i32::MIN);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_abs(self) -> Self::Output;
}

/// Performs checked exponentiation that returns `None` instead of wrapping around on overflow.
pub trait CheckedPow<Exp = Self>: Sized {
    ///  The resulting type after applying the exponentiation operation.
    type Output;

    /// Returns `self` raised to the power of `exp`, or `None` if overflow occurred.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedPow::checked_pow(2i32, 3u32), Some(8));
    /// assert_eq!(CheckedPow::checked_pow(i32::MAX, 2u32), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_pow(self, exp: Exp) -> Option<Self::Output>;

    /// Returns `self` raised to the power of `exp`, panicking if overflow occurred.
    ///
    /// # Panics
    ///
    /// Panics if overflow occurs when calculating the exponentiation.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::*;
    /// assert_eq!(CheckedPow::strict_pow(2i32, 3u32), 8);
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// # use gnum::num::ops::*;
    /// let _ = CheckedPow::strict_pow(i32::MAX, 2u32);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn strict_pow(self, exp: Exp) -> Self::Output;
}

macro_rules! impl_checked_ops {
    ($($t:ty),*) => {
        $(
            impl CheckedAdd for $t {
                #[inline]
                fn checked_add(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_add(self, rhs)
                }

                #[inline]
                fn strict_add(self, rhs: Self) -> Self::Output {
                    <$t>::strict_add(self, rhs)
                }

                #[inline]
                unsafe fn unchecked_add(self, rhs: Self) -> Self::Output {
                    unsafe { <$t>::unchecked_add(self, rhs) }
                }
            }

            impl CheckedSub for $t {
                #[inline]
                fn checked_sub(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_sub(self, rhs)
                }

                #[inline]
                fn strict_sub(self, rhs: Self) -> Self::Output {
                    <$t>::strict_sub(self, rhs)
                }

                #[inline]
                unsafe fn unchecked_sub(self, rhs: Self) -> Self::Output {
                    unsafe { <$t>::unchecked_sub(self, rhs) }
                }
            }

            impl CheckedMul for $t {
                #[inline]
                fn checked_mul(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_mul(self, rhs)
                }

                #[inline]
                fn strict_mul(self, rhs: Self) -> Self::Output {
                    <$t>::strict_mul(self, rhs)
                }

                #[inline]
                unsafe fn unchecked_mul(self, rhs: Self) -> Self::Output {
                    unsafe { <$t>::unchecked_mul(self, rhs) }
                }
            }

            impl CheckedDiv for $t {
                #[inline]
                fn checked_div(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_div(self, rhs)
                }

                #[inline]
                fn strict_div(self, rhs: Self) -> Self::Output {
                    <$t>::strict_div(self, rhs)
                }
            }

            impl CheckedRem for $t {
                #[inline]
                fn checked_rem(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_rem(self, rhs)
                }

                #[inline]
                fn strict_rem(self, rhs: Self) -> Self::Output {
                    <$t>::strict_rem(self, rhs)
                }
             }

            impl CheckedDivEuclid for $t {
                #[inline]
                fn checked_div_euclid(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_div_euclid(self, rhs)
                }

                #[inline]
                fn strict_div_euclid(self, rhs: Self) -> Self::Output {
                    <$t>::strict_div_euclid(self, rhs)
                }
            }

            impl CheckedRemEuclid for $t {
                #[inline]
                fn checked_rem_euclid(self, rhs: Self) -> Option<Self::Output> {
                    <$t>::checked_rem_euclid(self, rhs)
                }

                #[inline]
                fn strict_rem_euclid(self, rhs: Self) -> Self::Output {
                    <$t>::strict_rem_euclid(self, rhs)
                }
            }

            impl CheckedShl<u32> for $t {
                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self::Output> {
                    <$t>::checked_shl(self, rhs)
                }

                #[inline]
                fn strict_shl(self, rhs: u32) -> Self::Output {
                    <$t>::strict_shl(self, rhs)
                }

                #[inline]
                unsafe fn unchecked_shl(self, rhs: u32) -> Self::Output {
                    unsafe { <$t>::unchecked_shl(self, rhs) }
                }
            }

            impl CheckedShr<u32> for $t {
                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self::Output> {
                    <$t>::checked_shr(self, rhs)
                }

                #[inline]
                fn strict_shr(self, rhs: u32) -> Self::Output {
                    <$t>::strict_shr(self, rhs)
                }

                #[inline]
                unsafe fn unchecked_shr(self, rhs: u32) -> Self::Output {
                    unsafe { <$t>::unchecked_shr(self, rhs) }
                }
            }

            impl CheckedPow<u32> for $t {
                type Output = $t;

                #[inline]
                fn checked_pow(self, exp: u32) -> Option<Self::Output> {
                    <$t>::checked_pow(self, exp)
                }

                #[inline]
                fn strict_pow(self, exp: u32) -> Self::Output {
                    <$t>::strict_pow(self, exp)
                }
            }
        )*
    };
}

impl_checked_ops!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_checked_signed_ops {
    ($($t:ty),*) => {
        $(
            impl CheckedNeg for $t {
                #[inline]
                fn checked_neg(self) -> Option<Self::Output> {
                    <$t>::checked_neg(self)
                }

                #[inline]
                fn strict_neg(self) -> Self::Output {
                    <$t>::strict_neg(self)
                }

                #[inline]
                unsafe fn unchecked_neg(self) -> Self::Output {
                    unsafe { <$t>::unchecked_neg(self) }
                }
            }

            impl CheckedAbs for $t {
                type Output = $t;

                #[inline]
                fn checked_abs(self) -> Option<Self::Output> {
                    <$t>::checked_abs(self)
                }

                #[inline]
                fn strict_abs(self) -> Self::Output {
                    <$t>::strict_abs(self)
                }
            }
        )*
    };
}

impl_checked_signed_ops!(i8, i16, i32, i64, i128, isize);

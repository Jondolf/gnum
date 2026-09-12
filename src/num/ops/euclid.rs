/// Performs the [Euclidean division] operation.
///
/// [Euclidean division]: https://en.wikipedia.org/wiki/Euclidean_division
pub trait DivEuclid<Rhs = Self> {
    /// The resulting type after applying the `div_euclid` operation.
    type Output;

    /// Performs the `div_euclid` operation, which returns the quotient
    /// of Euclidean division of `self` by `rhs`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::DivEuclid;
    /// assert_eq!(DivEuclid::div_euclid(12, 5), 2);
    /// assert_eq!(DivEuclid::div_euclid(-12, 5), -3);
    /// assert_eq!(DivEuclid::div_euclid(12, -5), -2);
    /// assert_eq!(DivEuclid::div_euclid(-12, -5), 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn div_euclid(self, rhs: Rhs) -> Self::Output;
}

/// Performs the [Euclidean remainder] operation.
///
/// [Euclidean remainder]: https://en.wikipedia.org/wiki/Euclidean_division
pub trait RemEuclid<Rhs = Self> {
    /// The resulting type after applying the `rem_euclid` operation.
    type Output;

    /// Performs the `rem_euclid` operation, which returns the remainder
    /// of Euclidean division of `self` by `rhs`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::ops::RemEuclid;
    /// assert_eq!(RemEuclid::rem_euclid(12, 5), 2);
    /// assert_eq!(RemEuclid::rem_euclid(-12, 5), 3);
    /// assert_eq!(RemEuclid::rem_euclid(12, -5), 2);
    /// assert_eq!(RemEuclid::rem_euclid(-12, -5), 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "modulo", alias = "mod")]
    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_div_rem_euclid_int {
    ($($t:ty),*) => {
        $(
            impl DivEuclid for $t {
                type Output = Self;

                /// Returns the quotient of Euclidean division of `self` by `rhs`.
                ///
                /// This computes the integer `q` such that `self = q * rhs + r`, with
                /// `r = self.rem_euclid(rhs)` and `0 <= r < abs(rhs)`.
                ///
                /// In other words, the result is `self / rhs` rounded to the integer `q`
                /// such that `self >= q * rhs`.
                ///
                /// - If `self > 0`, this is equal to rounding towards zero (the default in Rust);
                /// - If `self < 0`, this is equal to rounding away from zero (towards +/- infinity).
                /// - If `rhs > 0`, this is equal to rounding towards -infinity;
                /// - If `rhs < 0`, this is equal to rounding towards +infinity.
                ///
                /// # Panics
                ///
                /// Panics if `rhs` is zero or if `self` is `Self::MIN` and `rhs` is -1.
                ///
                /// # Example
                ///
                /// ```
                /// # use gnum::num::ops::DivEuclid;
                /// let a = 7;
                /// let b = 4;
                ///
                /// assert_eq!(DivEuclid::div_euclid(a, b), 1); // 7 >= 4 * 1
                /// assert_eq!(DivEuclid::div_euclid(a, -b), -1); // 7 >= -4 * -1
                /// assert_eq!(DivEuclid::div_euclid(-a, b), -2); // -7 >= 4 * -2
                /// assert_eq!(DivEuclid::div_euclid(-a, -b), 2); // -7 >= -4 * 2
                /// ```
                #[inline]
                fn div_euclid(self, rhs: Self) -> Self::Output {
                    self.div_euclid(rhs)
                }
            }

            impl RemEuclid for $t {
                type Output = Self;

                /// Calculates the least nonnegative remainder of `self (mod rhs)`.
                ///
                /// This is done as if by the Euclidean division algorithm -- given
                /// `r = self.rem_euclid(rhs)`, the result satisfies
                /// `self = rhs * self.div_euclid(rhs) + r` and `0 <= r < abs(rhs)`.
                ///
                /// # Panics
                ///
                /// Panics if `rhs` is zero or if `self` is `Self::MIN` and `rhs` is -1.
                ///
                /// # Example
                ///
                /// ```
                /// # use gnum::num::ops::RemEuclid;
                /// let a = 7;
                /// let b = 4;
                ///
                /// assert_eq!(RemEuclid::rem_euclid(a, b), 3);
                /// assert_eq!(RemEuclid::rem_euclid(-a, b), 1);
                /// assert_eq!(RemEuclid::rem_euclid(a, -b), 3);
                /// assert_eq!(RemEuclid::rem_euclid(-a, -b), 1);
                /// ```
                ///
                /// This will panic:
                ///
                /// ```should_panic
                /// # use gnum::num::ops::RemEuclid;
                /// let _ = RemEuclid::rem_euclid(i16::MIN, -1);
                /// ```
                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self::Output {
                    self.rem_euclid(rhs)
                }
            }
        )*
    };
}

impl_div_rem_euclid_int!(i8, i16, i32, i64, isize);
impl_div_rem_euclid_int!(u8, u16, u32, u64, usize);

macro_rules! impl_div_rem_euclid_float {
    ($($t:ty),*) => {
        $(
            impl DivEuclid for $t {
                type Output = Self;

                /// Returns the Euclidean division of `self` by `rhs`.
                ///
                /// This computes the integer `n` such that `self = n * rhs + self.rem_euclid(rhs)`.
                /// In other words, the result is `self / rhs` rounded to the integer `n` such that `self >= n * rhs`.
                ///
                /// # Example
                ///
                /// ```
                /// # use gnum::num::ops::DivEuclid;
                /// let a = 7.0;
                /// let b = 4.0;
                ///
                /// assert_eq!(DivEuclid::div_euclid(a, b), 1.0); // 7 >= 4 * 1.0
                /// assert_eq!(DivEuclid::div_euclid(a, -b), -1.0); // 7 >= -4 * -1.0
                /// assert_eq!(DivEuclid::div_euclid(-a, b), -2.0); // -7 >= 4 * -2.0
                /// assert_eq!(DivEuclid::div_euclid(-a, -b), 2.0); // -7 >= -4 * 2.0
                /// ```
                #[inline]
                fn div_euclid(self, rhs: Self) -> Self::Output {
                    #[cfg(feature = "std")]
                    {
                        self.div_euclid(rhs)
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        let quotient = crate::num::Real::trunc(self / rhs);
                        let remainder = self % rhs;
                        if remainder < 0.0 {
                            if rhs > 0.0 {
                                quotient - 1.0
                            } else {
                                quotient + 1.0
                            }
                        } else {
                            quotient
                        }
                    }
                }
            }

            impl RemEuclid for $t {
                type Output = Self;

                /// Returns the least nonnegative remainder of `self (mod rhs)`.
                ///
                /// In particular, the return value `r` satisfied `0.0 <= r < abs(rhs)`.
                ///
                /// # Example
                ///
                /// ```
                /// # use gnum::num::ops::RemEuclid;
                /// let a = 7.0;
                /// let b = 4.0;
                ///
                /// assert_eq!(RemEuclid::rem_euclid(a, b), 3.0);
                /// assert_eq!(RemEuclid::rem_euclid(-a, b), 1.0);
                /// assert_eq!(RemEuclid::rem_euclid(a, -b), 3.0);
                /// assert_eq!(RemEuclid::rem_euclid(-a, -b), 1.0);
                /// ```
                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self::Output {
                    #[cfg(feature = "std")]
                    {
                        self.rem_euclid(rhs)
                    }
                    #[cfg(not(feature = "std"))]
                    {
                        let remainder = self % rhs;
                        if remainder < 0.0 {
                            remainder + rhs.abs()
                        } else {
                            remainder
                        }
                    }
                }
            }
        )*
    };
}

impl_div_rem_euclid_float!(f32, f64);

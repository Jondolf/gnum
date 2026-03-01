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
    /// assert_eq!(12.div_euclid(5), 2);
    /// assert_eq!((-12).div_euclid(5), -3);
    /// assert_eq!(12.div_euclid(-5), -3);
    /// assert_eq!((-12).div_euclid(-5), 2);
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
    /// assert_eq!(12.rem_euclid(5), 2);
    /// assert_eq!((-12).rem_euclid(5), 3);
    /// assert_eq!(12.rem_euclid(-5), -3);
    /// assert_eq!((-12).rem_euclid(-5), -2);
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
                /// let a = 7;
                /// let b = 4;
                ///
                /// assert_eq!(a.div_euclid(b), 1); // 7 >= 4 * 1
                /// assert_eq!(a.div_euclid(-b), -1); // 7 >= -4 * -1
                /// assert_eq!((-a).div_euclid(b), -2); // -7 >= 4 * -2
                /// assert_eq!((-a).div_euclid(-b), 2); // -7 >= -4 * 2
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
                /// let a = 7;
                /// let b = 4;
                ///
                /// assert_eq!(a.rem_euclid(b), 3);
                /// assert_eq!((-a).rem_euclid(b), 1);
                /// assert_eq!(a.rem_euclid(-b), 3);
                /// assert_eq!((-a).rem_euclid(-b), 1);
                /// ```
                ///
                /// This will panic:
                ///
                /// ```should_panic
                /// let _ = i16::MIN.rem_euclid(-1);
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
                /// let a = 7.0;
                /// let b = 4.0;
                ///
                /// assert_eq!(a.div_euclid(b), 1.0); // 7 >= 4 * 1.0
                /// assert_eq!(a.div_euclid(-b), -1.0); // 7 >= -4 * -1.0
                /// assert_eq!((-a).div_euclid(b), -2.0); // -7 >= 4 * -2.0
                /// assert_eq!((-a).div_euclid(-b), 2.0); // -7 >= -4 * 2.0
                /// ```
                #[inline]
                fn div_euclid(self, rhs: Self) -> Self::Output {
                    self.div_euclid(rhs)
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
                /// let a = 7.0;
                /// let b = 4.0;
                ///
                /// assert_eq!(a.rem_euclid(b), 3.0);
                /// assert_eq!((-a).rem_euclid(b), 1.0);
                /// assert_eq!(a.rem_euclid(-b), 3.0);
                /// assert_eq!((-a).rem_euclid(-b), 1.0);
                /// ```
                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self::Output {
                    self.rem_euclid(rhs)
                }
            }
        )*
    };
}

impl_div_rem_euclid_float!(f32, f64);

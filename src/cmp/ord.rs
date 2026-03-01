use super::NumEq;

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
/// For floating-point types, the [`min`](Self::min), [`max`](Self::max), and [`clamp`](Self::clamp)
/// methods follow the semantics of [`f32::min`], [`f32::max`], and [`f32::clamp`].
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
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn min(self, other: Self) -> Self;

    /// Returns the element-wise maximum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn max(self, other: Self) -> Self;

    /// Restrict each element to a certain interval.
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` on any element.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn clamp(self, min: Self, max: Self) -> Self;
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

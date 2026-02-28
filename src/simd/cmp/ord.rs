use std::simd::{LaneCount, MaskElement, SimdElement, SupportedLaneCount};

use crate::simd::SimdValue;

use super::SimdPartialEq;

/// Generalization of [`PartialOrd`] for SIMD types.
///
/// This is implemented for both scalar and SIMD types, so that
/// the same interface can be used for both. For scalar types,
/// the mask type is always [`bool`].8
pub trait SimdPartialOrd: SimdPartialEq {
    /// Test if each element is less than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_lt(self, other: Self) -> Self::Bool;

    /// Test if each element is less than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_le(self, other: Self) -> Self::Bool;

    /// Test if each element is greater than the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_gt(self, other: Self) -> Self::Bool;

    /// Test if each element is greater than or equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ge(self, other: Self) -> Self::Bool;
}

/// Generalization of [`Ord`] for SIMD types.
///
/// This is implemented for both scalar and SIMD types, so that
/// the same interface can be used for both. For scalar types,
/// the mask type is always [`bool`].
pub trait SimdOrd: SimdPartialOrd {
    /// Returns the element-wise minimum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_min(self, other: Self) -> Self;

    /// Returns the element-wise maximum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_max(self, other: Self) -> Self;

    /// Restrict each element to a certain interval.
    ///
    /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` on any element.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_simd_partial_ord_scalar {
    ($($t:ty),*) => {
        $(
            impl SimdPartialOrd for $t {
                #[inline]
                fn simd_lt(self, other: Self) -> Self::Bool {
                    self < other
                }

                #[inline]
                fn simd_le(self, other: Self) -> Self::Bool {
                    self <= other
                }

                #[inline]
                fn simd_gt(self, other: Self) -> Self::Bool {
                    self > other
                }

                #[inline]
                fn simd_ge(self, other: Self) -> Self::Bool {
                    self >= other
                }
            }
        )*
    };
}

impl_simd_partial_ord_scalar!(u8, u16, u32, u64, usize);
impl_simd_partial_ord_scalar!(i8, i16, i32, i64, isize);
impl_simd_partial_ord_scalar!(f32, f64);

macro_rules! impl_simd_ord_scalar {
    ($($t:ty),*) => {
        $(
            impl SimdOrd for $t {
                #[inline]
                fn simd_max(self, other: Self) -> Self {
                    self.max(other)
                }

                #[inline]
                fn simd_min(self, other: Self) -> Self {
                    self.min(other)
                }

                #[inline]
                fn simd_clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }
            }
        )*
    };
}

impl_simd_ord_scalar!(u8, u16, u32, u64, usize);
impl_simd_ord_scalar!(i8, i16, i32, i64, isize);

impl<T: SimdElement, const N: usize> SimdPartialOrd for core::simd::Simd<T, N>
where
    Self: SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialOrd,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_lt(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_lt(self, other)
    }

    #[inline]
    fn simd_le(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_le(self, other)
    }

    #[inline]
    fn simd_gt(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_gt(self, other)
    }

    #[inline]
    fn simd_ge(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_ge(self, other)
    }
}

impl<T: MaskElement, const N: usize> SimdPartialOrd for core::simd::Mask<T, N>
where
    Self: SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialOrd,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_lt(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_lt(self, other)
    }

    #[inline]
    fn simd_le(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_le(self, other)
    }

    #[inline]
    fn simd_gt(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_gt(self, other)
    }

    #[inline]
    fn simd_ge(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialOrd::simd_ge(self, other)
    }
}

impl<T: SimdElement, const N: usize> SimdOrd for core::simd::Simd<T, N>
where
    Self:
        SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask> + core::simd::cmp::SimdOrd,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_max(self, other: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_max(self, other)
    }

    #[inline]
    fn simd_min(self, other: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_min(self, other)
    }

    #[inline]
    fn simd_clamp(self, min: Self, max: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_clamp(self, min, max)
    }
}

impl<T: MaskElement, const N: usize> SimdOrd for core::simd::Mask<T, N>
where
    Self:
        SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask> + core::simd::cmp::SimdOrd,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_max(self, other: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_max(self, other)
    }

    #[inline]
    fn simd_min(self, other: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_min(self, other)
    }

    #[inline]
    fn simd_clamp(self, min: Self, max: Self) -> Self {
        core::simd::cmp::SimdOrd::simd_clamp(self, min, max)
    }
}

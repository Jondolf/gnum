use core::simd::{LaneCount, MaskElement, SimdElement, SupportedLaneCount};

use crate::simd::SimdValue;

/// Generalizattion of [`PartialEq`] for SIMD types.
///
/// This is implemented for both scalar and SIMD types, so that
/// the same interface can be used for both. For scalar types,
/// the mask type is always [`bool`].
pub trait SimdPartialEq: SimdValue {
    /// Test if each element is equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_eq(self, other: Self) -> Self::Bool;

    /// Test if each element is not equal to the corresponding element in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ne(self, other: Self) -> Self::Bool;
}

macro_rules! impl_simd_partial_eq_scalar {
    ($($t:ty),*) => {
        $(
            impl SimdPartialEq for $t {
                #[inline]
                fn simd_eq(self, other: Self) -> Self::Bool {
                    self == other
                }

                #[inline]
                fn simd_ne(self, other: Self) -> Self::Bool {
                    self != other
                }
            }
        )*
    };
}

impl_simd_partial_eq_scalar!(u8, u16, u32, u64, usize);
impl_simd_partial_eq_scalar!(i8, i16, i32, i64, isize);
impl_simd_partial_eq_scalar!(f32, f64);

impl<T: SimdElement, const N: usize> SimdPartialEq for core::simd::Simd<T, N>
where
    Self: SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_eq(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_eq(self, other)
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_ne(self, other)
    }
}

impl<T: MaskElement, const N: usize> SimdPartialEq for core::simd::Mask<T, N>
where
    Self: SimdValue<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialEq,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn simd_eq(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_eq(self, other)
    }

    #[inline]
    fn simd_ne(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_ne(self, other)
    }
}

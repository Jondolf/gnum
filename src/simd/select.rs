use std::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

use crate::simd::SimdBool;

/// Choose elements from two vectors using a mask.
///
/// For each element in the mask, choose the corresponding element from `true_values` if
/// that element mask is true, and `false_values` if that element mask is false.
pub trait Select<T>: SimdBool {
    /// Choose elements from two vectors.
    ///
    /// For each element in the mask, choose the corresponding element from `true_values` if
    /// that element mask is true, and `false_values` if that element mask is false.
    fn select(self, true_values: T, false_values: T) -> T;
}

impl<T> Select<T> for bool {
    #[inline]
    fn select(self, true_values: T, false_values: T) -> T {
        if self { true_values } else { false_values }
    }
}

impl<T: SimdElement, const N: usize> Select<Simd<T, N>> for Mask<T::Mask, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn select(
        self,
        true_values: core::simd::Simd<T, N>,
        false_values: core::simd::Simd<T, N>,
    ) -> core::simd::Simd<T, N> {
        self.select(true_values, false_values)
    }
}

impl<T: MaskElement, const N: usize> Select<Mask<T, N>> for Mask<T::Mask, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn select(
        self,
        true_values: core::simd::Mask<T, N>,
        false_values: core::simd::Mask<T, N>,
    ) -> core::simd::Mask<T, N> {
        self.select_mask(true_values, false_values)
    }
}

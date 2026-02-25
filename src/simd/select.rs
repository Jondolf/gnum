use std::simd::{LaneCount, Mask, Simd, SimdElement, SupportedLaneCount};

/// Choose elements from two vectors using a mask.
///
/// For each element in the mask, choose the corresponding element from `true_values` if
/// that element mask is true, and `false_values` if that element mask is false.
///
/// If the mask is `u64`, it's treated as a bitmask with the least significant bit
/// corresponding to the first element.
pub trait Select<T> {
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

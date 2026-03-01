use crate::simd::Select;
use core::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

impl<T: SimdElement, const N: usize> Select<Simd<T, N>> for Mask<T::Mask, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn select(self, true_values: Simd<T, N>, false_values: Simd<T, N>) -> Simd<T, N> {
        self.select(true_values, false_values)
    }
}

impl<T: MaskElement, const N: usize> Select<Mask<T, N>> for Mask<T::Mask, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn select(self, true_values: Mask<T, N>, false_values: Mask<T, N>) -> Mask<T, N> {
        self.select_mask(true_values, false_values)
    }
}

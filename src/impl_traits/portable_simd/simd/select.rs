use crate::simd::Select;
use core::simd::{Mask, MaskElement, Simd, SimdElement};

impl<T: SimdElement, const N: usize> Select<Simd<T, N>> for Mask<T::Mask, N> {
    #[inline]
    fn select(self, true_values: Simd<T, N>, false_values: Simd<T, N>) -> Simd<T, N> {
        core::simd::Select::select(self, true_values, false_values)
    }
}

impl<T: MaskElement, const N: usize> Select<Mask<T, N>> for Mask<T::Mask, N> {
    #[inline]
    fn select(self, true_values: Mask<T, N>, false_values: Mask<T, N>) -> Mask<T, N> {
        core::simd::Select::select(self, true_values, false_values)
    }
}

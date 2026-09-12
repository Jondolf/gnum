use crate::{Mask, Simd, SimdElement};
use gnum::simd::Select;

impl<T: SimdElement<N>, const N: usize> Select<Simd<T, N>> for Mask<T, N> {
    #[inline]
    fn select(self, true_values: Simd<T, N>, false_values: Simd<T, N>) -> Simd<T, N> {
        Simd::from_inner(self.0.select(true_values.0, false_values.0))
    }
}

impl<T: SimdElement<N>, const N: usize> Select<Mask<T, N>> for Mask<T, N> {
    #[inline]
    fn select(self, true_values: Mask<T, N>, false_values: Mask<T, N>) -> Mask<T, N> {
        Mask::from_inner(self.0.select(true_values.0, false_values.0))
    }
}

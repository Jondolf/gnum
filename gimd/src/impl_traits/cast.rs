use super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::num::NumCast;

impl<T: SimdElement<N>, U: SimdElement<N>, const N: usize> NumCast<Simd<U, N>> for Simd<T, N>
where
    BackendVector<T, N>: NumCast<BackendVector<U, N>>,
{
    #[inline]
    fn cast(self) -> Simd<U, N> {
        Simd::from_inner(self.0.cast())
    }
}

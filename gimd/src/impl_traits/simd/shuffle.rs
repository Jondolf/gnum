use super::super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::simd::{Shuffle4, Shuffle8};

impl<T: SimdElement<4>> Shuffle4 for Simd<T, 4>
where
    BackendVector<T, 4>: Shuffle4,
{
    #[inline]
    fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(self) -> Self {
        Self::from_inner(self.0.shuffle::<A, B, C, D>())
    }
}

impl<T: SimdElement<8>> Shuffle8 for Simd<T, 8>
where
    BackendVector<T, 8>: Shuffle8,
{
    #[inline]
    fn shuffle<
        const A: usize,
        const B: usize,
        const C: usize,
        const D: usize,
        const E: usize,
        const F: usize,
        const G: usize,
        const H: usize,
    >(
        self,
    ) -> Self {
        Self::from_inner(self.0.shuffle::<A, B, C, D, E, F, G, H>())
    }
}

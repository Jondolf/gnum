use crate::simd::{Shuffle4, Shuffle8, SimdLike};
use core::simd::{Simd, SimdElement, Swizzle};

struct Shuf4<const A: usize, const B: usize, const C: usize, const D: usize>;

impl<const A: usize, const B: usize, const C: usize, const D: usize> Swizzle<4>
    for Shuf4<A, B, C, D>
{
    const INDEX: [usize; 4] = [A, B, C, D];
}

struct Shuf8<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
>;

impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
> Swizzle<8> for Shuf8<A, B, C, D, E, F, G, H>
{
    const INDEX: [usize; 8] = [A, B, C, D, E, F, G, H];
}

impl<T: SimdElement> Shuffle4 for Simd<T, 4>
where
    Simd<T, 4>: SimdLike,
{
    #[inline(always)]
    fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(self) -> Self {
        Shuf4::<A, B, C, D>::swizzle(self)
    }
}

impl<T: SimdElement> Shuffle8 for Simd<T, 8>
where
    Simd<T, 8>: SimdLike,
{
    #[inline(always)]
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
        Shuf8::<A, B, C, D, E, F, G, H>::swizzle(self)
    }
}

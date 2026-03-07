use crate::simd::SimdLike;
use core::simd::{MaskElement, SimdElement};

impl<T: SimdElement + SimdLike<Element = T, Bool = bool>, const N: usize> SimdLike
    for core::simd::Simd<T, N>
{
    const LANES: usize = N;
    type Element = T;
    type Bool = core::simd::Mask<T::Mask, N>;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        Self::splat(val)
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        self[i]
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        self[i]
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self[i] = val;
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self[i] = val;
    }
}

impl<T: MaskElement + SimdLike<Element = T, Bool = bool>, const N: usize> SimdLike
    for core::simd::Mask<T, N>
{
    const LANES: usize = N;
    type Element = bool;
    type Bool = Self;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        Self::splat(val)
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        self.test(i)
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        self.test(i)
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.set(i, val);
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self.set(i, val);
    }
}

use crate::{
    num::{Int, Signed},
    simd::SimdLike,
};
use core::simd::{MaskElement, Simd, SimdElement};

impl<T: SimdElement + SimdLike<Element = T, Bool = bool>, const N: usize> SimdLike for Simd<T, N>
where
    Simd<T::Mask, N>: Int + Signed,
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
    fn replace(&mut self, i: usize, value: Self::Element) {
        self[i] = value;
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, value: Self::Element) {
        self[i] = value;
    }
}

impl<T: MaskElement + SimdLike<Element = T, Bool = bool>, const N: usize> SimdLike
    for core::simd::Mask<T, N>
where
    Simd<T, N>: Int + Signed,
{
    const LANES: usize = N;
    type Element = bool;
    type Bool = Self;

    #[inline]
    fn splat(value: Self::Element) -> Self {
        Self::splat(value)
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
    fn replace(&mut self, i: usize, value: Self::Element) {
        self.set(i, value);
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, value: Self::Element) {
        self.set(i, value);
    }
}

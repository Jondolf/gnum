use crate::simd::{SimdLike, Swizzle};
use core::simd::{Simd, SimdElement};

impl<T: SimdElement, const N: usize> Swizzle for Simd<T, N>
where
    Simd<T, N>: SimdLike<Element = T>,
{
    #[inline(always)]
    fn reverse(self) -> Self {
        self.reverse()
    }

    #[inline(always)]
    fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
        self.rotate_elements_left::<OFFSET>()
    }

    #[inline(always)]
    fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
        self.rotate_elements_right::<OFFSET>()
    }

    #[inline(always)]
    fn shift_elements_left<const OFFSET: usize>(self, padding: Self::Element) -> Self {
        self.shift_elements_left::<OFFSET>(padding)
    }

    #[inline(always)]
    fn shift_elements_right<const OFFSET: usize>(self, padding: Self::Element) -> Self {
        self.shift_elements_right::<OFFSET>(padding)
    }

    #[inline(always)]
    fn interleave(self, other: Self) -> (Self, Self) {
        self.interleave(other)
    }

    #[inline(always)]
    fn deinterleave(self, other: Self) -> (Self, Self) {
        self.deinterleave(other)
    }
}

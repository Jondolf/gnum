use super::super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::simd::Swizzle;

impl<T: SimdElement<N>, const N: usize> Swizzle for Simd<T, N>
where
    BackendVector<T, N>: Swizzle<Element = T>,
{
    #[inline]
    fn reverse(self) -> Self {
        Self::from_inner(self.0.reverse())
    }

    #[inline]
    fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
        Self::from_inner(self.0.rotate_elements_left::<OFFSET>())
    }

    #[inline]
    fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
        Self::from_inner(self.0.rotate_elements_right::<OFFSET>())
    }

    #[inline]
    fn shift_elements_left<const OFFSET: usize>(self, padding: T) -> Self {
        Self::from_inner(self.0.shift_elements_left::<OFFSET>(padding))
    }

    #[inline]
    fn shift_elements_right<const OFFSET: usize>(self, padding: T) -> Self {
        Self::from_inner(self.0.shift_elements_right::<OFFSET>(padding))
    }

    #[inline]
    fn interleave(self, other: Self) -> (Self, Self) {
        let (a, b) = self.0.interleave(other.0);
        (Self::from_inner(a), Self::from_inner(b))
    }

    #[inline]
    fn deinterleave(self, other: Self) -> (Self, Self) {
        let (a, b) = self.0.deinterleave(other.0);
        (Self::from_inner(a), Self::from_inner(b))
    }
}

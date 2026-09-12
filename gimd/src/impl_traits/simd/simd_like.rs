use crate::{Mask, Simd, SimdElement};
use gnum::simd::{MaskLike, SimdLike};

impl<T: SimdElement<N>, const N: usize> SimdLike for Simd<T, N> {
    const LANES: usize = N;
    type Element = T;
    type Bool = Mask<T, N>;
    type Array = [T; N];

    #[inline]
    fn splat(value: T) -> Self {
        Self::splat(value)
    }

    #[inline]
    fn to_array(self) -> [T; N] {
        self.to_array()
    }

    #[inline]
    fn extract(&self, i: usize) -> T {
        self[i]
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> T {
        unsafe { *self.as_array().get_unchecked(i) }
    }

    #[inline]
    fn replace(&mut self, i: usize, value: T) {
        self[i] = value;
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, value: T) {
        unsafe {
            *self.as_mut_array().get_unchecked_mut(i) = value;
        }
    }
}

impl<T: SimdElement<N>, const N: usize> SimdLike for Mask<T, N> {
    const LANES: usize = N;
    type Element = bool;
    type Bool = Self;
    type Array = [bool; N];

    #[inline]
    fn splat(value: bool) -> Self {
        Self::splat(value)
    }

    #[inline]
    fn to_array(self) -> [bool; N] {
        self.to_array()
    }

    #[inline]
    fn extract(&self, i: usize) -> bool {
        self.test(i)
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> bool {
        unsafe { self.0.test_unchecked(i) }
    }

    #[inline]
    fn replace(&mut self, i: usize, value: bool) {
        self.set(i, value);
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, value: bool) {
        unsafe {
            self.0.set_unchecked(i, value);
        }
    }
}

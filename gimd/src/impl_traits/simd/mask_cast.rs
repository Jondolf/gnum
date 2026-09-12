use super::super::BackendMask;
use crate::{Mask, SimdElement};
use gnum::simd::MaskCast;

impl<T: SimdElement<N>, const N: usize> MaskCast for Mask<T, N> {
    type Int = T::MaskInt;

    #[inline]
    fn from_int(value: Self::Int) -> Self {
        Self::from_inner(BackendMask::<T, N>::from_int(value))
    }

    #[inline]
    unsafe fn from_int_unchecked(value: Self::Int) -> Self {
        Self::from_inner(unsafe { BackendMask::<T, N>::from_int_unchecked(value) })
    }

    #[inline]
    fn to_int(self) -> Self::Int {
        self.0.to_int()
    }

    #[inline]
    fn cast<U: MaskCast<Int = Self::Int>>(self) -> U {
        U::from_int(self.to_int())
    }
}

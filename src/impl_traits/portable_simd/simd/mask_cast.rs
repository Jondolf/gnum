use crate::{
    num::{Int, Signed},
    simd::MaskCast,
};
use core::simd::{Mask, MaskElement, Simd};

impl<T: MaskElement, const N: usize> MaskCast for Mask<T, N>
where
    Simd<T, N>: Int + Signed,
{
    type Int = Simd<T, N>;

    #[inline(always)]
    fn from_int(value: Self::Int) -> Self {
        Self::from_simd(value)
    }

    #[inline(always)]
    unsafe fn from_int_unchecked(value: Self::Int) -> Self {
        unsafe { Self::from_simd_unchecked(value) }
    }

    #[inline(always)]
    fn to_int(self) -> Self::Int {
        self.to_simd()
    }

    #[inline(always)]
    fn cast<U: MaskCast<Int = Self::Int>>(self) -> U {
        U::from_int(self.to_int())
    }
}

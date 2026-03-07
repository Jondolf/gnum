use crate::{cmp::NumEq, simd::SimdLike};
use core::simd::{MaskElement, SimdElement};

impl<T: SimdElement, const N: usize> NumEq for core::simd::Simd<T, N>
where
    Self: SimdLike<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialEq,
{
    #[inline]
    fn num_eq(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_eq(self, other)
    }

    #[inline]
    fn num_ne(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_ne(self, other)
    }
}

impl<T: MaskElement, const N: usize> NumEq for core::simd::Mask<T, N>
where
    Self: SimdLike<Bool = <Self as core::simd::cmp::SimdPartialEq>::Mask>
        + core::simd::cmp::SimdPartialEq,
{
    #[inline]
    fn num_eq(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_eq(self, other)
    }

    #[inline]
    fn num_ne(self, other: Self) -> Self::Bool {
        core::simd::cmp::SimdPartialEq::simd_ne(self, other)
    }
}

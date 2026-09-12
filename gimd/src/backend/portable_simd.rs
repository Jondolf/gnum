use gnum::{
    num::{Int, Signed},
    simd::{MaskCast, Select, SimdLike},
};

use super::{RealElement, SimdElement, sealed::Sealed};
use core::simd::{Mask, Simd, SimdElement as CoreSimdElement};

impl<T: CoreSimdElement, const N: usize> Sealed<N> for T {}

impl<T, const N: usize> SimdElement<N> for T
where
    T: CoreSimdElement + core::fmt::Debug + SimdLike<Element = T, Bool = bool> + 'static,
    Simd<T, N>: SimdLike<Element = T, Bool = Mask<T::Mask, N>, Array = [T; N]>,
    Simd<T::Mask, N>: Int + Signed,
    Mask<T::Mask, N>: core::fmt::Debug
        + MaskCast<Int = Simd<T::Mask, N>>
        + Select<Simd<T, N>>
        + Select<Mask<T::Mask, N>>,
{
    type Vector = Simd<T, N>;
    type MaskInt = Simd<T::Mask, N>;
    type Mask = Mask<T::Mask, N>;

    #[inline]
    fn vector_as_array(vector: &Self::Vector) -> &[Self; N] {
        vector.as_array()
    }

    #[inline]
    fn vector_as_mut_array(vector: &mut Self::Vector) -> &mut [Self; N] {
        vector.as_mut_array()
    }

    #[inline]
    fn mask_from_array(array: [bool; N]) -> Self::Mask {
        Mask::from_array(array)
    }

    #[inline]
    fn mask_to_array(mask: Self::Mask) -> [bool; N] {
        mask.to_array()
    }
}

impl<const N: usize> RealElement<N> for f32
where
    f32: SimdElement<N>,
    i32: SimdElement<N>,
{
    type I32Element = i32;
}

impl<const N: usize> RealElement<N> for f64
where
    f64: SimdElement<N>,
    i32: SimdElement<N>,
{
    type I32Element = i32;
}

use super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::num::{NegOne, Num, One, Zero};

impl<T: SimdElement<N>, const N: usize> Zero for Simd<T, N>
where
    BackendVector<T, N>: Zero,
{
    const ZERO: Self = Self(<BackendVector<T, N> as Zero>::ZERO);
}

impl<T: SimdElement<N>, const N: usize> One for Simd<T, N>
where
    BackendVector<T, N>: One,
{
    const ONE: Self = Self(<BackendVector<T, N> as One>::ONE);
}

impl<T: SimdElement<N>, const N: usize> NegOne for Simd<T, N>
where
    BackendVector<T, N>: NegOne,
{
    const NEG_ONE: Self = Self(<BackendVector<T, N> as NegOne>::NEG_ONE);
}

impl<T: SimdElement<N>, const N: usize> Num for Simd<T, N>
where
    BackendVector<T, N>: Num,
{
    const MIN: Self = Self(<BackendVector<T, N> as Num>::MIN);
    const MAX: Self = Self(<BackendVector<T, N> as Num>::MAX);
}

use super::{BackendMask, BackendVector};
use crate::{Mask, Simd, SimdElement};
use gnum::cmp::{NumEq, NumOrd};

impl<T: SimdElement<N>, const N: usize> NumEq for Simd<T, N>
where
    BackendVector<T, N>: NumEq,
{
    #[inline]
    fn num_eq(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_eq(other.0))
    }

    #[inline]
    fn num_ne(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_ne(other.0))
    }
}

impl<T: SimdElement<N>, const N: usize> NumOrd for Simd<T, N>
where
    BackendVector<T, N>: NumOrd,
{
    #[inline]
    fn num_lt(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_lt(other.0))
    }

    #[inline]
    fn num_le(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_le(other.0))
    }

    #[inline]
    fn num_gt(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_gt(other.0))
    }

    #[inline]
    fn num_ge(self, other: Self) -> Self::Bool {
        Mask::from_inner(self.0.num_ge(other.0))
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self::from_inner(self.0.min(other.0))
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self::from_inner(self.0.max(other.0))
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::from_inner(self.0.clamp(min.0, max.0))
    }

    #[inline]
    fn min_fast(self, other: Self) -> Self {
        Self::from_inner(self.0.min_fast(other.0))
    }

    #[inline]
    fn max_fast(self, other: Self) -> Self {
        Self::from_inner(self.0.max_fast(other.0))
    }

    #[inline]
    fn clamp_fast(self, min: Self, max: Self) -> Self {
        Self::from_inner(self.0.clamp_fast(min.0, max.0))
    }
}

impl<T: SimdElement<N>, const N: usize> NumEq for Mask<T, N>
where
    BackendMask<T, N>: NumEq<Bool = BackendMask<T, N>>,
{
    #[inline]
    fn num_eq(self, other: Self) -> Self {
        Self::from_inner(self.0.num_eq(other.0))
    }

    #[inline]
    fn num_ne(self, other: Self) -> Self {
        Self::from_inner(self.0.num_ne(other.0))
    }
}

impl<T: SimdElement<N>, const N: usize> NumOrd for Mask<T, N>
where
    BackendMask<T, N>: NumOrd<Bool = BackendMask<T, N>>,
{
    #[inline]
    fn num_lt(self, other: Self) -> Self {
        Self::from_inner(self.0.num_lt(other.0))
    }

    #[inline]
    fn num_le(self, other: Self) -> Self {
        Self::from_inner(self.0.num_le(other.0))
    }

    #[inline]
    fn num_gt(self, other: Self) -> Self {
        Self::from_inner(self.0.num_gt(other.0))
    }

    #[inline]
    fn num_ge(self, other: Self) -> Self {
        Self::from_inner(self.0.num_ge(other.0))
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self::from_inner(self.0.min(other.0))
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self::from_inner(self.0.max(other.0))
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::from_inner(self.0.clamp(min.0, max.0))
    }

    #[inline]
    fn min_fast(self, other: Self) -> Self {
        Self::from_inner(self.0.min_fast(other.0))
    }

    #[inline]
    fn max_fast(self, other: Self) -> Self {
        Self::from_inner(self.0.max_fast(other.0))
    }

    #[inline]
    fn clamp_fast(self, min: Self, max: Self) -> Self {
        Self::from_inner(self.0.clamp_fast(min.0, max.0))
    }
}

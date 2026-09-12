use super::super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::simd::{Reduce, ReduceBitwise};

impl<T: SimdElement<N>, const N: usize> Reduce for Simd<T, N>
where
    BackendVector<T, N>: Reduce<Element = T>,
{
    #[inline]
    fn reduce_sum(self) -> T {
        self.0.reduce_sum()
    }

    #[inline]
    fn reduce_sum_stable(self) -> T {
        self.0.reduce_sum_stable()
    }

    #[inline]
    fn reduce_product(self) -> T {
        self.0.reduce_product()
    }

    #[inline]
    fn reduce_product_stable(self) -> T {
        self.0.reduce_product_stable()
    }

    #[inline]
    fn reduce_min(self) -> T {
        self.0.reduce_min()
    }

    #[inline]
    fn reduce_max(self) -> T {
        self.0.reduce_max()
    }
}

impl<T: SimdElement<N>, const N: usize> ReduceBitwise for Simd<T, N>
where
    BackendVector<T, N>: ReduceBitwise<Element = T>,
{
    #[inline]
    fn reduce_and(self) -> T {
        self.0.reduce_and()
    }

    #[inline]
    fn reduce_or(self) -> T {
        self.0.reduce_or()
    }

    #[inline]
    fn reduce_xor(self) -> T {
        self.0.reduce_xor()
    }
}

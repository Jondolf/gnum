use super::BackendVector;
use crate::{Mask, Simd, SimdElement};
use gnum::{
    num::{Num, Signed},
    simd::SimdLike,
};

type Unsigned<T> = <T as Signed>::Unsigned;

impl<T, const N: usize> Signed for Simd<T, N>
where
    T: SimdElement<N> + Signed,
    Unsigned<T>: SimdElement<N>,
    Self: SimdLike<Bool = Mask<T, N>>,
    Simd<Unsigned<T>, N>: Num,
    BackendVector<T, N>: Signed<Unsigned = BackendVector<Unsigned<T>, N>>,
{
    type Unsigned = Simd<Unsigned<T>, N>;

    #[inline]
    fn abs(self) -> Self {
        Self::from_inner(self.0.abs())
    }

    #[inline]
    fn abs_diff(self, rhs: Self) -> Self::Unsigned {
        Simd::from_inner(self.0.abs_diff(rhs.0))
    }

    #[inline]
    fn signum(self) -> Self {
        Self::from_inner(self.0.signum())
    }

    #[inline]
    fn is_positive(self) -> Self::Bool {
        Mask::from_inner(self.0.is_positive())
    }

    #[inline]
    fn is_negative(self) -> Self::Bool {
        Mask::from_inner(self.0.is_negative())
    }
}

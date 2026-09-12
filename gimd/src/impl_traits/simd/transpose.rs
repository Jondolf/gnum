use super::super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::simd::TransposeRows;

impl<T: SimdElement<N>, const N: usize> TransposeRows<N> for Simd<T, N>
where
    BackendVector<T, N>: TransposeRows<N>,
{
    #[inline]
    fn transpose_rows(rows: [Self; N]) -> [Self; N] {
        let rows = rows.map(Self::into_inner);
        BackendVector::<T, N>::transpose_rows(rows).map(Self::from_inner)
    }
}

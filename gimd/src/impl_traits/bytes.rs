use super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::ToBytes;

impl<T: SimdElement<N>, const N: usize> ToBytes for Simd<T, N>
where
    BackendVector<T, N>: ToBytes,
{
    type Bytes = <BackendVector<T, N> as ToBytes>::Bytes;

    #[inline]
    fn to_ne_bytes(self) -> Self::Bytes {
        self.0.to_ne_bytes()
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Bytes {
        self.0.to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        self.0.to_le_bytes()
    }

    #[inline]
    fn from_ne_bytes(bytes: Self::Bytes) -> Self {
        Self::from_inner(BackendVector::<T, N>::from_ne_bytes(bytes))
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        Self::from_inner(BackendVector::<T, N>::from_be_bytes(bytes))
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        Self::from_inner(BackendVector::<T, N>::from_le_bytes(bytes))
    }
}

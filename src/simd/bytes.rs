use crate::traits::ToBytes;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

macro_rules! impl_to_bytes_simd {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> ToBytes for Simd<$t, N>
            where
                Self: core::simd::ToBytes,
                LaneCount<N>: SupportedLaneCount,
            {
                type Bytes = <Self as core::simd::ToBytes>::Bytes;

                #[inline]
                fn to_ne_bytes(self) -> Self::Bytes {
                    core::simd::ToBytes::to_ne_bytes(self)
                }
                #[inline]
                fn to_be_bytes(self) -> Self::Bytes {
                    core::simd::ToBytes::to_be_bytes(self)
                }
                #[inline]
                fn to_le_bytes(self) -> Self::Bytes {
                    core::simd::ToBytes::to_le_bytes(self)
                }
                #[inline]
                fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                    core::simd::ToBytes::from_ne_bytes(bytes)
                }
                #[inline]
                fn from_be_bytes(bytes: Self::Bytes) -> Self {
                    core::simd::ToBytes::from_be_bytes(bytes)
                }
                #[inline]
                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    core::simd::ToBytes::from_le_bytes(bytes)
                }
            }
        )*
    };
}

impl_to_bytes_simd!(u8, u16, u32, u64, usize);
impl_to_bytes_simd!(i8, i16, i32, i64, isize);
impl_to_bytes_simd!(f32, f64);

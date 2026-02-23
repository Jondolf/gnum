use super::ToBytes;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

impl<const N: usize> ToBytes for Simd<f64, N>
where
    Self: core::simd::ToBytes,
    LaneCount<N>: SupportedLaneCount,
{
    type Bytes = <Self as core::simd::ToBytes>::Bytes;

    fn to_ne_bytes(self) -> Self::Bytes {
        core::simd::ToBytes::to_ne_bytes(self)
    }
    fn to_be_bytes(self) -> Self::Bytes {
        core::simd::ToBytes::to_be_bytes(self)
    }
    fn to_le_bytes(self) -> Self::Bytes {
        core::simd::ToBytes::to_le_bytes(self)
    }
    fn from_ne_bytes(bytes: Self::Bytes) -> Self {
        core::simd::ToBytes::from_ne_bytes(bytes)
    }
    fn from_be_bytes(bytes: Self::Bytes) -> Self {
        core::simd::ToBytes::from_be_bytes(bytes)
    }
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        core::simd::ToBytes::from_le_bytes(bytes)
    }
}

use crate::traits::Int;
use std::simd::{LaneCount, Simd, SupportedLaneCount, num::SimdInt};

impl<const N: usize> Int for Simd<i32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type U32 = Simd<u32, N>;

    const BITS: u32 = 8;

    #[inline]
    fn count_ones(self) -> Self::U32 {
        SimdInt::count_ones(self)
    }
    #[inline]
    fn count_zeros(self) -> Self::U32 {
        SimdInt::count_zeros(self)
    }
    #[inline]
    fn leading_zeros(self) -> Self::U32 {
        SimdInt::leading_zeros(self)
    }
    #[inline]
    fn trailing_zeros(self) -> Self::U32 {
        SimdInt::trailing_zeros(self)
    }
    #[inline]
    fn leading_ones(self) -> Self::U32 {
        SimdInt::leading_ones(self)
    }
    #[inline]
    fn trailing_ones(self) -> Self::U32 {
        SimdInt::trailing_ones(self)
    }
    #[inline]
    fn rotate_left(self, n: Self::U32) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].rotate_left(n[i]);
        }
        result
    }
    #[inline]
    fn rotate_right(self, n: Self::U32) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].rotate_right(n[i]);
        }
        result
    }
    #[inline]
    fn swap_bytes(self) -> Self {
        SimdInt::swap_bytes(self)
    }
    #[inline]
    fn reverse_bits(self) -> Self {
        SimdInt::reverse_bits(self)
    }
    #[inline]
    fn from_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(not(target_endian = "big"))]
        {
            SimdInt::swap_bytes(self)
        }
    }
    #[inline]
    fn from_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            SimdInt::swap_bytes(self)
        }
    }
    #[inline]
    fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(not(target_endian = "big"))]
        {
            SimdInt::swap_bytes(self)
        }
    }
    #[inline]
    fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            SimdInt::swap_bytes(self)
        }
    }
    #[inline]
    fn ilog(self, base: Self) -> Self::U32 {
        let mut result = Self::U32::default();
        for i in 0..Self::LEN {
            result[i] = self[i].ilog(base[i]);
        }
        result
    }
    #[inline]
    fn ilog2(self) -> Self::U32 {
        let mut result = Self::U32::default();
        for i in 0..Self::LEN {
            result[i] = self[i].ilog2();
        }
        result
    }
    #[inline]
    fn ilog10(self) -> Self::U32 {
        let mut result = Self::U32::default();
        for i in 0..Self::LEN {
            result[i] = self[i].ilog10();
        }
        result
    }
    #[inline]
    fn pow(self, exp: Self::U32) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].pow(exp[i]);
        }
        result
    }
    #[inline]
    fn isqrt(self) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].isqrt();
        }
        result
    }
    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].div_euclid(rhs[i]);
        }
        result
    }
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..Self::LEN {
            result[i] = result[i].rem_euclid(rhs[i]);
        }
        result
    }
}

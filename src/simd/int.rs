use crate::traits::Int;
use std::simd::{
    LaneCount, Simd, SupportedLaneCount,
    num::{SimdInt, SimdUint},
};

macro_rules! impl_int_simd {
    ($($int:ty => $uint:ty),*) => {
        $(
            impl<const N: usize> Int for Simd<$int, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Unsigned = Simd<$uint, N>;

                const BITS: u32 = <$int>::BITS;

                #[inline]
                fn count_ones(self) -> Self::Unsigned {
                    SimdInt::count_ones(self)
                }
                #[inline]
                fn count_zeros(self) -> Self::Unsigned {
                    SimdInt::count_zeros(self)
                }
                #[inline]
                fn leading_zeros(self) -> Self::Unsigned {
                    SimdInt::leading_zeros(self)
                }
                #[inline]
                fn trailing_zeros(self) -> Self::Unsigned {
                    SimdInt::trailing_zeros(self)
                }
                #[inline]
                fn leading_ones(self) -> Self::Unsigned {
                    SimdInt::leading_ones(self)
                }
                #[inline]
                fn trailing_ones(self) -> Self::Unsigned {
                    SimdInt::trailing_ones(self)
                }
                #[inline]
                fn rotate_left(self, n: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].rotate_left(n[i] as u32);
                    }
                    result
                }
                #[inline]
                fn rotate_right(self, n: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].rotate_right(n[i] as u32);
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
                fn ilog(self, base: Self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog(base[i]) as $uint;
                    }
                    result
                }
                #[inline]
                fn ilog2(self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog2() as $uint;
                    }
                    result
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog10() as $uint;
                    }
                    result
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].pow(exp[i] as u32);
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
            }
        )*
    };
}

impl_int_simd!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

macro_rules! impl_uint_simd {
    ($($uint:ty),*) => {
        $(
            impl<const N: usize> Int for Simd<$uint, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Unsigned = Self;

                const BITS: u32 = <$uint>::BITS;

                #[inline]
                fn count_ones(self) -> Self::Unsigned {
                    SimdUint::count_ones(self)
                }
                #[inline]
                fn count_zeros(self) -> Self::Unsigned {
                    SimdUint::count_zeros(self)
                }
                #[inline]
                fn leading_zeros(self) -> Self::Unsigned {
                    SimdUint::leading_zeros(self)
                }
                #[inline]
                fn trailing_zeros(self) -> Self::Unsigned {
                    SimdUint::trailing_zeros(self)
                }
                #[inline]
                fn leading_ones(self) -> Self::Unsigned {
                    SimdUint::leading_ones(self)
                }
                #[inline]
                fn trailing_ones(self) -> Self::Unsigned {
                    SimdUint::trailing_ones(self)
                }
                #[inline]
                fn rotate_left(self, n: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].rotate_left(n[i] as u32);
                    }
                    result
                }
                #[inline]
                fn rotate_right(self, n: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].rotate_right(n[i] as u32);
                    }
                    result
                }
                #[inline]
                fn swap_bytes(self) -> Self {
                    SimdUint::swap_bytes(self)
                }
                #[inline]
                fn reverse_bits(self) -> Self {
                    SimdUint::reverse_bits(self)
                }
                #[inline]
                fn from_be(self) -> Self {
                    #[cfg(target_endian = "big")]
                    {
                        self
                    }
                    #[cfg(not(target_endian = "big"))]
                    {
                        SimdUint::swap_bytes(self)
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
                        SimdUint::swap_bytes(self)
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
                        SimdUint::swap_bytes(self)
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
                        SimdUint::swap_bytes(self)
                    }
                }
                #[inline]
                fn ilog(self, base: Self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog(base[i]) as $uint;
                    }
                    result
                }
                #[inline]
                fn ilog2(self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog2() as $uint;
                    }
                    result
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    let mut result = Self::Unsigned::default();
                    for i in 0..Self::LEN {
                        result[i] = self[i].ilog10() as $uint;
                    }
                    result
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].pow(exp[i] as u32);
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
            }
        )*
    };
}

impl_uint_simd!(u8, u16, u32, u64, usize);

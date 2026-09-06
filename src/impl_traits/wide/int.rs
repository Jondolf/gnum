use crate::num::Int;
use wide::bytemuck;
use wide::*;

macro_rules! ilog10 {
    ($simd:ident, $value:expr; $($power:expr),+ $(,)?) => {{
        let x = $value;
        let mut result = <$simd>::ZERO;
        $(
            result += x
                .simd_ge(<$simd>::splat($power))
                .select(<$simd>::ONE, <$simd>::ZERO);
        )+
        result
    }};
    (u8, $simd:ident, $value:expr) => {
        ilog10!($simd, $value; 10, 100)
    };
    (u16, $simd:ident, $value:expr) => {
        ilog10!($simd, $value; 10, 100, 1_000, 10_000)
    };
    (u32, $simd:ident, $value:expr) => {
        ilog10!($simd, $value; 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000)
    };
    (u64, $simd:ident, $value:expr) => {
        ilog10!($simd, $value;
            10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000,
            1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000,
            10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000,
            10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
            10_000_000_000_000_000_000)
    };
}

macro_rules! impl_int {
    ($($simd:ident => $elem:ty, $usimd:ident, $uelem:ident, $ssimd:ident);* $(;)?) => {
        $(
            impl Int for $simd {
                type Unsigned = $usimd;
                type Signed = $ssimd;

                const BITS: u32 = <$elem>::BITS;

                #[inline]
                fn count_ones(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.count_ones() as $uelem)
                }
                #[inline]
                fn count_zeros(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.count_zeros() as $uelem)
                }
                #[inline]
                fn leading_zeros(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.leading_zeros() as $uelem)
                }
                #[inline]
                fn trailing_zeros(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.trailing_zeros() as $uelem)
                }
                #[inline]
                fn leading_ones(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.leading_ones() as $uelem)
                }
                #[inline]
                fn trailing_ones(self) -> Self::Unsigned {
                    map_lanes_to!($usimd, self, |x: $elem| x.trailing_ones() as $uelem)
                }
                #[inline]
                fn rotate_left(self, n: Self::Unsigned) -> Self {
                    let x: $usimd = bytemuck::cast(self);
                    bytemuck::cast((x << n) | (x >> (<$usimd>::ZERO - n)))
                }
                #[inline]
                fn rotate_right(self, n: Self::Unsigned) -> Self {
                    zip_lanes!($simd, self, n, |x: $elem, n: $uelem| x.rotate_right(n as u32))
                }
                #[inline]
                fn swap_bytes(self) -> Self {
                    <$simd>::new(self.to_array().map(<$elem>::swap_bytes))
                }
                #[inline]
                fn reverse_bits(self) -> Self {
                    <$simd>::new(self.to_array().map(<$elem>::reverse_bits))
                }
                #[inline]
                fn from_be(self) -> Self {
                    #[cfg(target_endian = "big")]
                    {
                        self
                    }
                    #[cfg(not(target_endian = "big"))]
                    {
                        self.swap_bytes()
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
                        self.swap_bytes()
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
                        self.swap_bytes()
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
                        self.swap_bytes()
                    }
                }
                #[inline]
                fn cast_unsigned(self) -> Self::Unsigned {
                    bytemuck::cast(self)
                }
                #[inline]
                fn cast_signed(self) -> Self::Signed {
                    bytemuck::cast(self)
                }
                #[inline]
                fn ilog(self, base: Self) -> Self::Unsigned {
                    zip_lanes_to!($usimd, self, base, |a: $elem, b: $elem| a.ilog(b) as $uelem)
                }
                #[inline]
                fn ilog2(self) -> Self::Unsigned {
                    #[cfg(all(
                        any(target_arch = "x86", target_arch = "x86_64"),
                        target_feature = "avx2"
                    ))]
                    {
                        if self.simd_le(<$simd>::ZERO).any() {
                            panic!("argument of integer logarithm must be positive");
                        }
                        let mut x: $usimd = bytemuck::cast(self);
                        let mut shift = 1;
                        while shift < <$elem>::BITS {
                            x |= x >> shift;
                            shift <<= 1;
                        }
                        let max = <$uelem>::MAX;
                        x -= (x >> 1) & <$usimd>::splat(max / 3);
                        x = (x & <$usimd>::splat(max / 5))
                            + ((x >> 2) & <$usimd>::splat(max / 5));
                        x = (x + (x >> 4)) & <$usimd>::splat(max / 17);
                        let ones = (x * <$usimd>::splat(max / 255)) >> (<$elem>::BITS - 8);
                        return ones - <$usimd>::ONE;
                    }
                    #[cfg(not(all(
                        any(target_arch = "x86", target_arch = "x86_64"),
                        target_feature = "avx2"
                    )))]
                    {
                        map_lanes_to!($usimd, self, |x: $elem| x.ilog2() as $uelem)
                    }
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    if self.simd_le(<$simd>::ZERO).any() {
                        panic!("argument of integer logarithm must be positive");
                    }
                    let x: $usimd = bytemuck::cast(self);
                    ilog10!($uelem, $usimd, x)
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    #[cfg(debug_assertions)]
                    {
                        zip_lanes!($simd, self, exp, |x: $elem, exp: $uelem| x.pow(exp as u32))
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        let mut base = self;
                        let mut exp = exp & <$usimd>::splat(u32::MAX as $uelem);
                        let mut result = <$simd>::ONE;
                        while exp.simd_ne(<$usimd>::ZERO).any() {
                            let odd = (exp & <$usimd>::ONE).simd_ne(<$usimd>::ZERO);
                            result = odd.select(result * base, result);
                            exp >>= 1;
                            base *= base;
                        }
                        result
                    }
                }
                #[inline]
                fn isqrt(self) -> Self {
                    if self.simd_lt(<$simd>::ZERO).any() {
                        panic!("argument of integer square root cannot be negative");
                    }

                    let mut remainder: $usimd = bytemuck::cast(self);
                    let mut result = <$usimd>::ZERO;
                    let mut bit = <$usimd>::splat((1 as $uelem) << (<$elem>::BITS - 2));
                    let iterations = <$elem>::BITS / 2;
                    let mut i = 0;
                    while i < iterations {
                        let trial = result + bit;
                        let take = remainder.simd_ge(trial);
                        remainder = take.select(remainder - trial, remainder);
                        result = take.select((result >> 1) + bit, result >> 1);
                        bit >>= 2;
                        i += 1;
                    }
                    bytemuck::cast(result)
                }
            }
        )*
    };
}

impl_int!(
    i8x16 => i8, u8x16, u8, i8x16; i8x32 => i8, u8x32, u8, i8x32; i8x64 => i8, u8x64, u8, i8x64;
    i16x8 => i16, u16x8, u16, i16x8; i16x16 => i16, u16x16, u16, i16x16; i16x32 => i16, u16x32, u16, i16x32;
    i32x4 => i32, u32x4, u32, i32x4; i32x8 => i32, u32x8, u32, i32x8; i32x16 => i32, u32x16, u32, i32x16;
    i64x2 => i64, u64x2, u64, i64x2; i64x4 => i64, u64x4, u64, i64x4; i64x8 => i64, u64x8, u64, i64x8;
    u8x16 => u8, u8x16, u8, i8x16; u8x32 => u8, u8x32, u8, i8x32; u8x64 => u8, u8x64, u8, i8x64;
    u16x8 => u16, u16x8, u16, i16x8; u16x16 => u16, u16x16, u16, i16x16; u16x32 => u16, u16x32, u16, i16x32;
    u32x4 => u32, u32x4, u32, i32x4; u32x8 => u32, u32x8, u32, i32x8; u32x16 => u32, u32x16, u32, i32x16;
    u64x2 => u64, u64x2, u64, i64x2; u64x4 => u64, u64x4, u64, i64x4; u64x8 => u64, u64x8, u64, i64x8;
);

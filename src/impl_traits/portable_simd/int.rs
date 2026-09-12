#[allow(unused_imports, reason = "release-only vector exponentiation")]
use crate::num::One;
use crate::{
    num::{Int, Zero},
    simd::Select,
};
use core::simd::{
    Simd,
    cmp::{SimdPartialEq, SimdPartialOrd},
    num::{SimdInt, SimdUint},
};

macro_rules! ilog10 {
    ($uint:ty, $value:expr; $($power:expr),+ $(,)?) => {{
        let x: Simd<$uint, N> = $value;
        let mut result = Simd::splat(0);
        $(
            result += x
                .simd_ge(Simd::splat($power))
                .select(Simd::splat(1), Simd::splat(0));
        )+
        result
    }};
    (u8, $value:expr) => {
        ilog10!(u8, $value; 10, 100)
    };
    (u16, $value:expr) => {
        ilog10!(u16, $value; 10, 100, 1_000, 10_000)
    };
    (u32, $value:expr) => {
        ilog10!(u32, $value; 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000)
    };
    (u64, $value:expr) => {
        ilog10!(u64, $value;
            10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000,
            1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000,
            10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000,
            10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
            10_000_000_000_000_000_000)
    };
    (usize, $value:expr) => {{
        #[cfg(target_pointer_width = "32")]
        { ilog10!(usize, $value; 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000) }
        #[cfg(target_pointer_width = "64")]
        { ilog10!(usize, $value;
            10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000,
            1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000,
            10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000,
            10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
            10_000_000_000_000_000_000) }
    }};
}

#[allow(unused_macros, reason = "release-only fast path")]
macro_rules! vector_pow {
    ($base:expr, $exp:expr) => {{
        let mut base = $base;
        let mut exp = $exp;
        let mut result = Self::ONE;
        while exp.simd_ne(Self::Unsigned::ZERO).any() {
            let odd = (exp & Self::Unsigned::ONE).simd_ne(Self::Unsigned::ZERO);
            result = odd.select(result * base, result);
            exp >>= 1;
            base *= base;
        }
        result
    }};
}

macro_rules! unsigned_isqrt {
    ($value:expr, $uint:ty) => {{
        let mut remainder: Simd<$uint, N> = $value;
        let mut result = Simd::splat(0);
        let mut bit = Simd::splat((1 as $uint) << (<$uint>::BITS - 2));
        let mut i = 0;
        while i < <$uint>::BITS / 2 {
            let trial = result + bit;
            let take = remainder.simd_ge(trial);
            remainder = take.select(remainder - trial, remainder);
            result = take.select((result >> 1) + bit, result >> 1);
            bit >>= 2;
            i += 1;
        }
        result
    }};
}

macro_rules! impl_int_simd {
    ($($int:ident => $uint:ident),*) => {
        $(
            impl<const N: usize> Int for Simd<$int, N> {
                type Unsigned = Simd<$uint, N>;
                type Signed = Self;

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
                fn cast_unsigned(self) -> Self::Unsigned {
                    SimdInt::cast(self)
                }
                #[inline]
                fn cast_signed(self) -> Self::Signed {
                    self
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
                    if <$uint>::BITS == 64 {
                        let mut result = Self::Unsigned::default();
                        for i in 0..Self::LEN {
                            result[i] = self[i].ilog2() as $uint;
                        }
                        return result;
                    }
                    if self.simd_le(Self::ZERO).any() {
                        panic!("argument of integer logarithm must be positive");
                    }
                    Simd::splat((<$int>::BITS - 1) as $uint) - SimdInt::leading_zeros(self)
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    if <$uint>::BITS == 64 {
                        let mut result = Self::Unsigned::default();
                        for i in 0..Self::LEN {
                            result[i] = self[i].ilog10() as $uint;
                        }
                        return result;
                    }
                    if self.simd_le(Self::ZERO).any() {
                        panic!("argument of integer logarithm must be positive");
                    }
                    ilog10!($uint, SimdInt::cast(self))
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    #[cfg(debug_assertions)]
                    {
                        let mut result = self;
                        for i in 0..Self::LEN {
                            result[i] = result[i].pow(exp[i] as u32);
                        }
                        result
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        if <$uint>::BITS == 64 {
                            let mut result = self;
                            for i in 0..Self::LEN {
                                result[i] = result[i].pow(exp[i] as u32);
                            }
                            result
                        } else {
                            vector_pow!(self, exp)
                        }
                    }
                }
                #[inline]
                fn isqrt(self) -> Self {
                    if <$uint>::BITS == 64 {
                        let mut result = self;
                        for i in 0..Self::LEN {
                            result[i] = result[i].isqrt();
                        }
                        return result;
                    }
                    if self.simd_lt(Self::ZERO).any() {
                        panic!("argument of integer square root cannot be negative");
                    }
                    SimdUint::cast(unsigned_isqrt!(SimdInt::cast(self), $uint))
                }
            }
        )*
    };
}

impl_int_simd!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

macro_rules! impl_uint_simd {
    ($($uint:ident => $int:ident),*) => {
        $(
            impl<const N: usize> Int for Simd<$uint, N> {
                type Unsigned = Self;
                type Signed = Simd<$int, N>;

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
                fn cast_unsigned(self) -> Self::Unsigned {
                    self
                }
                #[inline]
                fn cast_signed(self) -> Self::Signed {
                    SimdUint::cast(self)
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
                    if <$uint>::BITS == 64 {
                        let mut result = Self::Unsigned::default();
                        for i in 0..Self::LEN {
                            result[i] = self[i].ilog2() as $uint;
                        }
                        return result;
                    }
                    if self.simd_eq(Self::ZERO).any() {
                        panic!("argument of integer logarithm must be positive");
                    }
                    Simd::splat((<$uint>::BITS - 1) as $uint) - SimdUint::leading_zeros(self)
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    if <$uint>::BITS == 64 {
                        let mut result = Self::Unsigned::default();
                        for i in 0..Self::LEN {
                            result[i] = self[i].ilog10() as $uint;
                        }
                        return result;
                    }
                    if self.simd_eq(Self::ZERO).any() {
                        panic!("argument of integer logarithm must be positive");
                    }
                    ilog10!($uint, self)
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    #[cfg(debug_assertions)]
                    {
                        let mut result = self;
                        for i in 0..Self::LEN {
                            result[i] = result[i].pow(exp[i] as u32);
                        }
                        result
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        if <$uint>::BITS == 64 {
                            let mut result = self;
                            for i in 0..Self::LEN {
                                result[i] = result[i].pow(exp[i] as u32);
                            }
                            result
                        } else {
                            vector_pow!(self, exp)
                        }
                    }
                }
                #[inline]
                fn isqrt(self) -> Self {
                    if <$uint>::BITS == 64 {
                        let mut result = self;
                        for i in 0..Self::LEN {
                            result[i] = result[i].isqrt();
                        }
                        result
                    } else {
                        unsigned_isqrt!(self, $uint)
                    }
                }
            }
        )*
    };
}

impl_uint_simd!(u8 => i8, u16 => i16, u32 => i32, u64 => i64, usize => isize);

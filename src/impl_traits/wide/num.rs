use crate::num::{NegOne, Num, One, Zero};
use wide::*;

macro_rules! impl_num {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl Num for $simd {
                const MIN: Self = Self::splat(<$elem>::MIN);
                const MAX: Self = Self::splat(<$elem>::MAX);
            }

            impl Zero for $simd {
                const ZERO: Self = Self::splat(0 as $elem);
            }

            impl One for $simd {
                const ONE: Self = Self::splat(1 as $elem);
            }
        )*
    };
}

impl_num!(
    f32x4 => f32, f32x8 => f32, f32x16 => f32,
    f64x2 => f64, f64x4 => f64, f64x8 => f64,
    i8x16 => i8, i8x32 => i8, i8x64 => i8,
    i16x8 => i16, i16x16 => i16, i16x32 => i16,
    i32x4 => i32, i32x8 => i32, i32x16 => i32,
    i64x2 => i64, i64x4 => i64, i64x8 => i64,
    u8x16 => u8, u8x32 => u8, u8x64 => u8,
    u16x8 => u16, u16x16 => u16, u16x32 => u16,
    u32x4 => u32, u32x8 => u32, u32x16 => u32,
    u64x2 => u64, u64x4 => u64, u64x8 => u64,
);

macro_rules! impl_neg_one {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl NegOne for $simd {
                const NEG_ONE: Self = Self::splat(-1 as $elem);
            }
        )*
    };
}

impl_neg_one!(
    f32x4 => f32, f32x8 => f32, f32x16 => f32,
    f64x2 => f64, f64x4 => f64, f64x8 => f64,
    i8x16 => i8, i8x32 => i8, i8x64 => i8,
    i16x8 => i16, i16x16 => i16, i16x32 => i16,
    i32x4 => i32, i32x8 => i32, i32x16 => i32,
    i64x2 => i64, i64x4 => i64, i64x8 => i64,
);

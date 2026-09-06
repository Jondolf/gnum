use crate::num::SimdValue;
use wide::*;

macro_rules! impl_simd_value {
    ($($simd:ident),* $(,)?) => {
        $(
            impl SimdValue for $simd {}
        )*
    };
}

impl_simd_value!(
    f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i8x64, i16x8, i16x16, i16x32, i32x4,
    i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16,
    u64x2, u64x4, u64x8, u8x64,
);

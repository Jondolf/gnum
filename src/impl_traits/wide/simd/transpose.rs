use crate::simd::{Swizzle, TransposeRows, blocked_transpose};
use wide::*;

#[inline(always)]
fn generic<T: Swizzle + Copy, const N: usize>(rows: [T; N]) -> [T; N] {
    crate::simd::generic_transpose(rows)
}

macro_rules! impl_generic {
    ($($simd:ident, $lanes:literal);+ $(;)?) => {$(
        impl TransposeRows<$lanes> for $simd {
            #[inline]
            fn transpose_rows(rows: [Self; $lanes]) -> [Self; $lanes] {
                generic(rows)
            }
        }
    )+};
}

macro_rules! impl_inherent {
    ($($simd:ident, $lanes:literal);+ $(;)?) => {$(
        impl TransposeRows<$lanes> for $simd {
            #[inline]
            fn transpose_rows(rows: [Self; $lanes]) -> [Self; $lanes] {
                <$simd>::transpose(rows)
            }
        }
    )+};
}

macro_rules! impl_inherent_on {
    ($($simd:ident, $lanes:literal, $feature:literal);+ $(;)?) => {$(
        impl TransposeRows<$lanes> for $simd {
            #[inline]
            fn transpose_rows(rows: [Self; $lanes]) -> [Self; $lanes] {
                #[cfg(target_feature = $feature)]
                return <$simd>::transpose(rows);
                #[cfg(not(target_feature = $feature))]
                return generic(rows);
            }
        }
    )+};
}

macro_rules! blocked_fn {
    ($name:ident, $simd:ident, $lanes:literal, $block:ident, $block_lanes:literal, $blocks:literal, $transpose:path) => {
        #[inline(always)]
        fn $name(rows: [$simd; $lanes]) -> [$simd; $lanes] {
            blocked_transpose::<$simd, $block, $lanes, $block_lanes, $blocks>(
                rows,
                bytemuck::cast,
                bytemuck::cast,
                $transpose,
            )
        }
    };
}

impl_generic!(u8x16, 16; i8x16, 16);

impl_inherent!(
    f32x4, 4;
    f64x2, 2;
    i16x8, 8; u16x8, 8;
    i32x4, 4; u32x4, 4;
    i64x2, 2; u64x2, 2
);

impl_inherent_on!(
    f32x8, 8, "avx";
    f64x4, 4, "avx";
    i32x8, 8, "avx2"; u32x8, 8, "avx2";
    i64x4, 4, "avx2"; u64x4, 4, "avx2"
);

blocked_fn!(transpose_u8x32, u8x32, 32, u8x16, 16, 2, generic);
blocked_fn!(transpose_i8x32, i8x32, 32, i8x16, 16, 2, generic);
blocked_fn!(transpose_u8x64, u8x64, 64, u8x16, 16, 4, generic);
blocked_fn!(transpose_i8x64, i8x64, 64, i8x16, 16, 4, generic);

blocked_fn!(transpose_u16x16, u16x16, 16, u16x8, 8, 2, u16x8::transpose);
blocked_fn!(transpose_i16x16, i16x16, 16, i16x8, 8, 2, i16x8::transpose);
blocked_fn!(transpose_u16x32, u16x32, 32, u16x8, 8, 4, u16x8::transpose);
blocked_fn!(transpose_i16x32, i16x32, 32, i16x8, 8, 4, i16x8::transpose);

#[cfg(target_feature = "avx")]
blocked_fn!(
    transpose_f32x16_by_8,
    f32x16,
    16,
    f32x8,
    8,
    2,
    f32x8::transpose
);
#[cfg(not(target_feature = "avx2"))]
blocked_fn!(
    transpose_u32x16_by_4,
    u32x16,
    16,
    u32x4,
    4,
    4,
    u32x4::transpose
);
#[cfg(not(target_feature = "avx2"))]
blocked_fn!(
    transpose_i32x16_by_4,
    i32x16,
    16,
    i32x4,
    4,
    4,
    i32x4::transpose
);
#[cfg(target_feature = "avx2")]
blocked_fn!(
    transpose_u32x16_by_8,
    u32x16,
    16,
    u32x8,
    8,
    2,
    u32x8::transpose
);
#[cfg(target_feature = "avx2")]
blocked_fn!(
    transpose_i32x16_by_8,
    i32x16,
    16,
    i32x8,
    8,
    2,
    i32x8::transpose
);

#[inline(always)]
fn best_f64x4(rows: [f64x4; 4]) -> [f64x4; 4] {
    #[cfg(target_feature = "avx")]
    return f64x4::transpose(rows);
    #[cfg(not(target_feature = "avx"))]
    return generic(rows);
}

#[inline(always)]
fn best_u64x4(rows: [u64x4; 4]) -> [u64x4; 4] {
    #[cfg(target_feature = "avx2")]
    return u64x4::transpose(rows);
    #[cfg(not(target_feature = "avx2"))]
    return generic(rows);
}

#[inline(always)]
fn best_i64x4(rows: [i64x4; 4]) -> [i64x4; 4] {
    #[cfg(target_feature = "avx2")]
    return i64x4::transpose(rows);
    #[cfg(not(target_feature = "avx2"))]
    return generic(rows);
}

blocked_fn!(transpose_f64x8, f64x8, 8, f64x4, 4, 2, best_f64x4);
blocked_fn!(transpose_u64x8, u64x8, 8, u64x4, 4, 2, best_u64x4);
blocked_fn!(transpose_i64x8, i64x8, 8, i64x4, 4, 2, best_i64x4);

macro_rules! impl_blocked {
    ($($simd:ident, $lanes:literal, $function:ident);+ $(;)?) => {$(
        impl TransposeRows<$lanes> for $simd {
            #[inline]
            fn transpose_rows(rows: [Self; $lanes]) -> [Self; $lanes] {
                $function(rows)
            }
        }
    )+};
}

impl_blocked!(
    u8x32, 32, transpose_u8x32; i8x32, 32, transpose_i8x32;
    u8x64, 64, transpose_u8x64; i8x64, 64, transpose_i8x64;
    u16x16, 16, transpose_u16x16; i16x16, 16, transpose_i16x16;
    u16x32, 32, transpose_u16x32; i16x32, 32, transpose_i16x32;
    f64x8, 8, transpose_f64x8;
    u64x8, 8, transpose_u64x8; i64x8, 8, transpose_i64x8
);

impl TransposeRows<16> for f32x16 {
    #[inline]
    fn transpose_rows(rows: [Self; 16]) -> [Self; 16] {
        #[cfg(target_feature = "avx")]
        return transpose_f32x16_by_8(rows);
        #[cfg(not(target_feature = "avx"))]
        return f32x16::transpose(rows);
    }
}

macro_rules! impl_32x16 {
    ($simd:ident, $by_4:ident, $by_8:ident) => {
        impl TransposeRows<16> for $simd {
            #[inline]
            fn transpose_rows(rows: [Self; 16]) -> [Self; 16] {
                #[cfg(target_feature = "avx2")]
                return $by_8(rows);
                #[cfg(not(target_feature = "avx2"))]
                return $by_4(rows);
            }
        }
    };
}

impl_32x16!(u32x16, transpose_u32x16_by_4, transpose_u32x16_by_8);
impl_32x16!(i32x16, transpose_i32x16_by_4, transpose_i32x16_by_8);

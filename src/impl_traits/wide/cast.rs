use crate::num::NumCast;
use wide::*;

/// Casts each lane of `$v` from `$src` to `$dst`, with the same semantics as `as`.
macro_rules! cast_lanes {
    // Same-width signed/unsigned integer casts preserve the bit pattern.
    (i8x16 => $se:ty, u8x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u8x16 => $se:ty, i8x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i8x32 => $se:ty, u8x32 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u8x32 => $se:ty, i8x32 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i8x64 => $se:ty, u8x64 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u8x64 => $se:ty, i8x64 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i16x8 => $se:ty, u16x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u16x8 => $se:ty, i16x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i16x16 => $se:ty, u16x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u16x16 => $se:ty, i16x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i16x32 => $se:ty, u16x32 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u16x32 => $se:ty, i16x32 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i32x4 => $se:ty, u32x4 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u32x4 => $se:ty, i32x4 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i32x8 => $se:ty, u32x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u32x8 => $se:ty, i32x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i32x16 => $se:ty, u32x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u32x16 => $se:ty, i32x16 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i64x2 => $se:ty, u64x2 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u64x2 => $se:ty, i64x2 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i64x4 => $se:ty, u64x4 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u64x4 => $se:ty, i64x4 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (i64x8 => $se:ty, u64x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    (u64x8 => $se:ty, i64x8 => $de:ty, $v:expr) => {
        bytemuck::cast($v)
    };
    // Widening and truncating conversions are handled by `wide` explicitly.
    (i8x16 => $se:ty, i16x16 => $de:ty, $v:expr) => {
        i16x16::from($v)
    };
    (u8x16 => $se:ty, i16x16 => $de:ty, $v:expr) => {
        i16x16::from($v)
    };
    (u8x16 => $se:ty, u16x16 => $de:ty, $v:expr) => {
        u16x16::from($v)
    };
    (i8x32 => $se:ty, i16x32 => $de:ty, $v:expr) => {
        i16x32::from($v)
    };
    (i16x8 => $se:ty, i32x8 => $de:ty, $v:expr) => {
        i32x8::from($v)
    };
    (u16x8 => $se:ty, i32x8 => $de:ty, $v:expr) => {
        i32x8::from_u16x8($v)
    };
    (u16x8 => $se:ty, u32x8 => $de:ty, $v:expr) => {
        u32x8::from($v)
    };
    (i16x16 => $se:ty, i32x16 => $de:ty, $v:expr) => {
        i32x16::from($v)
    };
    (u16x16 => $se:ty, u32x16 => $de:ty, $v:expr) => {
        u32x16::from($v)
    };
    (i16x16 => $se:ty, i8x16 => $de:ty, $v:expr) => {
        i8x16::from_i16x16_truncate($v)
    };
    (i16x16 => $se:ty, u8x16 => $de:ty, $v:expr) => {
        bytemuck::cast(i8x16::from_i16x16_truncate($v))
    };
    (u16x16 => $se:ty, i8x16 => $de:ty, $v:expr) => {
        i8x16::from_i16x16_truncate(bytemuck::cast($v))
    };
    (u16x16 => $se:ty, u8x16 => $de:ty, $v:expr) => {
        bytemuck::cast(i8x16::from_i16x16_truncate(bytemuck::cast($v)))
    };
    (i32x8 => $se:ty, i16x8 => $de:ty, $v:expr) => {
        i16x8::from_i32x8_truncate($v)
    };
    (i32x8 => $se:ty, u16x8 => $de:ty, $v:expr) => {
        bytemuck::cast(i16x8::from_i32x8_truncate($v))
    };
    (u32x8 => $se:ty, i16x8 => $de:ty, $v:expr) => {
        i16x8::from_i32x8_truncate(bytemuck::cast($v))
    };
    (u32x8 => $se:ty, u16x8 => $de:ty, $v:expr) => {
        bytemuck::cast(i16x8::from_i32x8_truncate(bytemuck::cast($v)))
    };
    // Float-to-int casts truncate the float to an integer.
    (f32x4 => $se:ty, i32x4 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    (f32x8 => $se:ty, i32x8 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    (f32x16 => $se:ty, i32x16 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    (f64x2 => $se:ty, i64x2 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    (f64x4 => $se:ty, i64x4 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    (f64x8 => $se:ty, i64x8 => $de:ty, $v:expr) => {
        $v.trunc_int()
    };
    // Int-to-float casts round the integer to the nearest representable float,
    // with ties rounding to even.
    (i32x4 => $se:ty, f32x4 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    (i32x8 => $se:ty, f32x8 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    (i32x16 => $se:ty, f32x16 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    (i64x2 => $se:ty, f64x2 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    (i64x4 => $se:ty, f64x4 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    (i64x8 => $se:ty, f64x8 => $de:ty, $v:expr) => {
        $v.round_float()
    };
    // Every i32 is exactly representable as f64.
    (i32x4 => $se:ty, f64x4 => $de:ty, $v:expr) => {
        f64x4::from_i32x4($v)
    };
    (i32x8 => $se:ty, f64x8 => $de:ty, $v:expr) => {
        f64x8::from_i32x8($v)
    };
    // Fall back to casting each lane individually.
    ($src:ident => $se:ty, $dst:ident => $de:ty, $v:expr) => {
        map_lanes_to!($dst, $v, |x| x as $de)
    };
}

/// Implements [`NumCast`] between every pair of the given vector types.
///
/// All types in one invocation must have the same number of lanes.
macro_rules! impl_num_cast {
    ($($simd:ident => $elem:ty),+ $(,)?) => {
        impl_num_cast!(@src [$($simd => $elem),+] ; $($simd => $elem),+);
    };
    (@src $list:tt ; $($src:ident => $src_elem:ty),+) => {
        $(impl_num_cast!(@dst $src => $src_elem ; $list);)+
    };
    (@dst $src:ident => $src_elem:ty ; [$($dst:ident => $dst_elem:ty),+]) => {
        $(
            impl NumCast<$dst> for $src {
                #[inline(always)]
                fn cast(self) -> $dst {
                    cast_lanes!($src => $src_elem, $dst => $dst_elem, self)
                }
            }
        )+
    };
}

// 2 lanes
impl_num_cast!(f64x2 => f64, i64x2 => i64, u64x2 => u64);

// 4 lanes
impl_num_cast!(
    f32x4 => f32, f64x4 => f64,
    i32x4 => i32, i64x4 => i64,
    u32x4 => u32, u64x4 => u64,
);

// 8 lanes
impl_num_cast!(
    f32x8 => f32, f64x8 => f64,
    i16x8 => i16, i32x8 => i32, i64x8 => i64,
    u16x8 => u16, u32x8 => u32, u64x8 => u64,
);

// 16 lanes
impl_num_cast!(
    f32x16 => f32,
    i8x16 => i8, i16x16 => i16, i32x16 => i32,
    u8x16 => u8, u16x16 => u16, u32x16 => u32,
);

// 32 lanes
impl_num_cast!(
    i8x32 => i8, i16x32 => i16,
    u8x32 => u8, u16x32 => u16,
);

// 64 lanes
impl_num_cast!(i8x64 => i8, u8x64 => u8);

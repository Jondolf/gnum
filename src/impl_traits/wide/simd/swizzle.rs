use crate::simd::Swizzle;
use wide::*;

macro_rules! native_interleave {
    (u8x16, $a:expr, $b:expr) => {{ (u8x16::unpack_low($a, $b), u8x16::unpack_high($a, $b)) }};
    (i8x16, $a:expr, $b:expr) => {{
        let a: u8x16 = bytemuck::cast($a);
        let b: u8x16 = bytemuck::cast($b);
        let (lo, hi) = (u8x16::unpack_low(a, b), u8x16::unpack_high(a, b));
        (bytemuck::cast(lo), bytemuck::cast(hi))
    }};
    ($simd:ident, $uint:ident, $a:expr, $b:expr) => {{
        let a: $uint = bytemuck::cast($a);
        let b: $uint = bytemuck::cast($b);
        (
            bytemuck::cast(a.unpack_lo(b)),
            bytemuck::cast(a.unpack_hi(b)),
        )
    }};
}

#[inline(always)]
fn reverse_arr<T: Copy, const N: usize>(a: [T; N]) -> [T; N] {
    let mut out = a;
    out.reverse();
    out
}

#[inline(always)]
fn rotate_left_arr<T: Copy, const N: usize, const OFFSET: usize>(a: [T; N]) -> [T; N] {
    let offset = OFFSET % N;
    let mut out = a;
    let mut i = 0;
    while i < N {
        out[i] = a[(i + offset) % N];
        i += 1;
    }
    out
}

#[inline(always)]
fn rotate_right_arr<T: Copy, const N: usize, const OFFSET: usize>(a: [T; N]) -> [T; N] {
    let offset = N - OFFSET % N;
    let mut out = a;
    let mut i = 0;
    while i < N {
        out[i] = a[(i + offset) % N];
        i += 1;
    }
    out
}

#[inline(always)]
fn shift_left_arr<T: Copy, const N: usize, const OFFSET: usize>(a: [T; N], padding: T) -> [T; N] {
    let mut out = [padding; N];
    let mut i = 0;
    while i + OFFSET < N {
        out[i] = a[i + OFFSET];
        i += 1;
    }
    out
}

#[inline(always)]
fn shift_right_arr<T: Copy, const N: usize, const OFFSET: usize>(a: [T; N], padding: T) -> [T; N] {
    let mut out = [padding; N];
    let mut i = OFFSET;
    while i < N {
        out[i] = a[i - OFFSET];
        i += 1;
    }
    out
}

#[inline(always)]
fn concat<T: Copy, const N: usize>(a: &[T; N], b: &[T; N], index: usize) -> T {
    if index < N { a[index] } else { b[index - N] }
}

#[inline(always)]
fn interleave_arr<T: Copy, const N: usize>(a: [T; N], b: [T; N]) -> ([T; N], [T; N]) {
    let mut lo = a;
    let mut hi = a;
    let mut i = 0;
    while i < N {
        let lo_dst = i;
        lo[i] = concat(&a, &b, lo_dst / 2 + (lo_dst % 2) * N);
        let hi_dst = i + N;
        hi[i] = concat(&a, &b, hi_dst / 2 + (hi_dst % 2) * N);
        i += 1;
    }
    (lo, hi)
}

#[inline(always)]
fn deinterleave_arr<T: Copy, const N: usize>(a: [T; N], b: [T; N]) -> ([T; N], [T; N]) {
    let mut even = a;
    let mut odd = a;
    let mut i = 0;
    while i < N {
        even[i] = concat(&a, &b, i * 2);
        odd[i] = concat(&a, &b, i * 2 + 1);
        i += 1;
    }
    (even, odd)
}

macro_rules! impl_swizzle {
    ($($simd:ident => $elem:ty, $lanes:literal);* $(;)?) => {
        $(
            impl Swizzle for $simd {
                #[inline(always)]
                fn reverse(self) -> Self {
                    <$simd>::new(reverse_arr(self.to_array()))
                }

                #[inline(always)]
                fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
                    <$simd>::new(rotate_left_arr::<$elem, $lanes, OFFSET>(self.to_array()))
                }

                #[inline(always)]
                fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
                    <$simd>::new(rotate_right_arr::<$elem, $lanes, OFFSET>(self.to_array()))
                }

                #[inline(always)]
                fn shift_elements_left<const OFFSET: usize>(self, padding: $elem) -> Self {
                    <$simd>::new(shift_left_arr::<$elem, $lanes, OFFSET>(self.to_array(), padding))
                }

                #[inline(always)]
                fn shift_elements_right<const OFFSET: usize>(self, padding: $elem) -> Self {
                    <$simd>::new(shift_right_arr::<$elem, $lanes, OFFSET>(self.to_array(), padding))
                }

                #[inline(always)]
                fn interleave(self, other: Self) -> (Self, Self) {
                    let (lo, hi) = interleave_arr(self.to_array(), other.to_array());
                    (<$simd>::new(lo), <$simd>::new(hi))
                }

                #[inline(always)]
                fn deinterleave(self, other: Self) -> (Self, Self) {
                    let (even, odd) = deinterleave_arr(self.to_array(), other.to_array());
                    (<$simd>::new(even), <$simd>::new(odd))
                }
            }
        )*
    };
}

macro_rules! impl_swizzle_native_interleave {
    ($($simd:ident => $elem:ty, $lanes:literal, $($uint:ident)?);* $(;)?) => {
        $(
            impl Swizzle for $simd {
                #[inline(always)]
                fn reverse(self) -> Self {
                    <$simd>::new(reverse_arr(self.to_array()))
                }

                #[inline(always)]
                fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
                    <$simd>::new(rotate_left_arr::<$elem, $lanes, OFFSET>(self.to_array()))
                }

                #[inline(always)]
                fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
                    <$simd>::new(rotate_right_arr::<$elem, $lanes, OFFSET>(self.to_array()))
                }

                #[inline(always)]
                fn shift_elements_left<const OFFSET: usize>(self, padding: $elem) -> Self {
                    <$simd>::new(shift_left_arr::<$elem, $lanes, OFFSET>(self.to_array(), padding))
                }

                #[inline(always)]
                fn shift_elements_right<const OFFSET: usize>(self, padding: $elem) -> Self {
                    <$simd>::new(shift_right_arr::<$elem, $lanes, OFFSET>(self.to_array(), padding))
                }

                #[inline(always)]
                fn interleave(self, other: Self) -> (Self, Self) {
                    native_interleave!($simd $(, $uint)?, self, other)
                }

                #[inline(always)]
                fn deinterleave(self, other: Self) -> (Self, Self) {
                    let (even, odd) = deinterleave_arr(self.to_array(), other.to_array());
                    (<$simd>::new(even), <$simd>::new(odd))
                }
            }
        )*
    };
}

impl_swizzle_native_interleave!(
    u8x16 => u8, 16,;
    i8x16 => i8, 16,;
    f32x4 => f32, 4, u32x4; f32x8 => f32, 8, u32x8; f32x16 => f32, 16, u32x16;
    f64x2 => f64, 2, u64x2; f64x4 => f64, 4, u64x4; f64x8 => f64, 8, u64x8;
    i32x4 => i32, 4, u32x4; i32x8 => i32, 8, u32x8; i32x16 => i32, 16, u32x16;
    u32x4 => u32, 4, u32x4; u32x8 => u32, 8, u32x8; u32x16 => u32, 16, u32x16;
    i64x2 => i64, 2, u64x2; i64x4 => i64, 4, u64x4; i64x8 => i64, 8, u64x8;
    u64x2 => u64, 2, u64x2; u64x4 => u64, 4, u64x4; u64x8 => u64, 8, u64x8;
);

impl_swizzle!(
    i8x32 => i8, 32; i8x64 => i8, 64;
    i16x8 => i16, 8; i16x16 => i16, 16; i16x32 => i16, 32;
    u8x32 => u8, 32; u8x64 => u8, 64;
    u16x8 => u16, 8; u16x16 => u16, 16; u16x32 => u16, 32;
);

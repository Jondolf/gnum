use crate::simd::SimdLike;
use wide::*;

macro_rules! impl_simd_like {
    ($($simd:ident => $elem:ty, $lanes:literal);* $(;)?) => {
        $(
            impl SimdLike for $simd {
                const LANES: usize = $lanes;
                type Element = $elem;
                type Bool = Self;
                type Array = [$elem; $lanes];

                #[inline]
                fn splat(value: Self::Element) -> Self {
                    <$simd>::splat(value)
                }

                #[inline]
                fn to_array(self) -> Self::Array {
                    self.to_array()
                }

                #[inline]
                fn extract(&self, i: usize) -> Self::Element {
                    self.as_array()[i]
                }

                #[inline]
                unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                    unsafe { *self.as_array().get_unchecked(i) }
                }

                #[inline]
                fn replace(&mut self, i: usize, value: Self::Element) {
                    self.as_mut_array()[i] = value;
                }

                #[inline]
                unsafe fn replace_unchecked(&mut self, i: usize, value: Self::Element) {
                    unsafe { *self.as_mut_array().get_unchecked_mut(i) = value };
                }
            }
        )*
    };
}

impl_simd_like!(
    f32x4 => f32, 4; f32x8 => f32, 8; f32x16 => f32, 16;
    f64x2 => f64, 2; f64x4 => f64, 4; f64x8 => f64, 8;
    i8x16 => i8, 16; i8x32 => i8, 32; i8x64 => i8, 64;
    i16x8 => i16, 8; i16x16 => i16, 16; i16x32 => i16, 32;
    i32x4 => i32, 4; i32x8 => i32, 8; i32x16 => i32, 16;
    i64x2 => i64, 2; i64x4 => i64, 4; i64x8 => i64, 8;
    u8x16 => u8, 16; u8x32 => u8, 32; u8x64 => u8, 64;
    u16x8 => u16, 8; u16x16 => u16, 16; u16x32 => u16, 32;
    u32x4 => u32, 4; u32x8 => u32, 8; u32x16 => u32, 16;
    u64x2 => u64, 2; u64x4 => u64, 4; u64x8 => u64, 8;
);

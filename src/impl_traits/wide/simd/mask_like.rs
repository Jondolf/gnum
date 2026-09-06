use crate::simd::MaskLike;
use wide::*;

macro_rules! impl_mask_like {
    ($($simd:ident => $elem:ty, $true:expr, $false:expr);* $(;)?) => {
        $(
            impl MaskLike for $simd {
                const TRUE: Self = Self::splat($true);
                const FALSE: Self = Self::splat($false);

                #[inline(always)]
                fn test(&self, index: usize) -> bool {
                    assert!(
                        index < <$simd as crate::simd::SimdLike>::LANES,
                        "mask index out of bounds"
                    );
                    (self.to_bitmask() >> index) & 1 == 1
                }

                #[inline(always)]
                unsafe fn test_unchecked(&self, index: usize) -> bool {
                    (self.to_bitmask() >> index) & 1 == 1
                }

                #[inline(always)]
                fn set(&mut self, index: usize, value: bool) {
                    self.as_mut_array()[index] = if value { $true } else { $false };
                }

                #[inline(always)]
                unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
                    unsafe {
                        *self.as_mut_array().get_unchecked_mut(index) =
                            if value { $true } else { $false };
                    }
                }

                #[inline(always)]
                fn to_bitmask(self) -> u64 {
                    self.to_bitmask() as u64
                }

                #[inline(always)]
                fn all(self) -> bool {
                    self.all()
                }

                #[inline(always)]
                fn any(self) -> bool {
                    self.any()
                }
            }
        )*
    };
}

impl_mask_like!(
    f32x4 => f32, f32::from_bits(u32::MAX), 0.0;
    f32x8 => f32, f32::from_bits(u32::MAX), 0.0;
    f32x16 => f32, f32::from_bits(u32::MAX), 0.0;
    f64x2 => f64, f64::from_bits(u64::MAX), 0.0;
    f64x4 => f64, f64::from_bits(u64::MAX), 0.0;
    f64x8 => f64, f64::from_bits(u64::MAX), 0.0;
    i8x16 => i8, -1, 0; i8x32 => i8, -1, 0; i8x64 => i8, -1, 0;
    i16x8 => i16, -1, 0; i16x16 => i16, -1, 0; i16x32 => i16, -1, 0;
    i32x4 => i32, -1, 0; i32x8 => i32, -1, 0; i32x16 => i32, -1, 0;
    i64x2 => i64, -1, 0; i64x4 => i64, -1, 0; i64x8 => i64, -1, 0;
    u8x16 => u8, u8::MAX, 0; u8x32 => u8, u8::MAX, 0; u8x64 => u8, u8::MAX, 0;
    u16x8 => u16, u16::MAX, 0; u16x16 => u16, u16::MAX, 0; u16x32 => u16, u16::MAX, 0;
    u32x4 => u32, u32::MAX, 0; u32x8 => u32, u32::MAX, 0; u32x16 => u32, u32::MAX, 0;
    u64x2 => u64, u64::MAX, 0; u64x4 => u64, u64::MAX, 0; u64x8 => u64, u64::MAX, 0;
);

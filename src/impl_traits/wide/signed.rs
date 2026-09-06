use crate::num::Signed;
use wide::*;

macro_rules! impl_signed_int {
    ($($simd:ident => $elem:ty, $usimd:ident, $uelem:ty);* $(;)?) => {
        $(
            impl Signed for $simd {
                type Unsigned = $usimd;

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }

                #[inline]
                fn abs_diff(self, rhs: Self) -> Self::Unsigned {
                    let a: $usimd = self.cast_unsigned();
                    let b: $usimd = rhs.cast_unsigned();
                    self.simd_ge(rhs).select(a - b, b - a)
                }

                #[inline]
                fn signum(self) -> Self {
                    self.signum()
                }

                #[inline]
                fn is_positive(self) -> Self::Bool {
                    self.simd_gt(<$simd>::splat(0))
                }

                #[inline]
                fn is_negative(self) -> Self::Bool {
                    self.simd_lt(<$simd>::splat(0))
                }
            }
        )*
    };
}

impl_signed_int!(
    i8x16 => i8, u8x16, u8; i8x32 => i8, u8x32, u8; i8x64 => i8, u8x64, u8;
    i16x8 => i16, u16x8, u16; i16x16 => i16, u16x16, u16; i16x32 => i16, u16x32, u16;
    i32x4 => i32, u32x4, u32; i32x8 => i32, u32x8, u32; i32x16 => i32, u32x16, u32;
    i64x2 => i64, u64x2, u64; i64x4 => i64, u64x4, u64; i64x8 => i64, u64x8, u64;
);

macro_rules! impl_signed_float {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl Signed for $simd {
                type Unsigned = Self;

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }

                #[inline]
                fn abs_diff(self, rhs: Self) -> Self {
                    (self - rhs).abs()
                }

                #[inline]
                fn signum(self) -> Self {
                    // `wide` 1.7 vectorizes this, but returns an all-ones NaN,
                    // unlike the core scalar implementation.
                    self.is_nan()
                        .select(<$simd>::splat(<$elem>::NAN), Self::ONE.copysign(self))
                }

                #[inline]
                fn is_positive(self) -> Self::Bool {
                    self.simd_gt(Self::ZERO)
                }

                #[inline]
                fn is_negative(self) -> Self::Bool {
                    self.simd_lt(Self::ZERO)
                }
            }
        )*
    };
}

impl_signed_float!(f32x4 => f32, f32x8 => f32, f32x16 => f32, f64x2 => f64, f64x4 => f64, f64x8 => f64);

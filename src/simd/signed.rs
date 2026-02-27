use crate::{
    cmp::SimdPartialOrd,
    traits::{Signed, Zero},
};
use core::simd::{LaneCount, Simd, SupportedLaneCount};
use std::simd::num::{SimdFloat, SimdInt};

macro_rules! impl_signed_simd {
    ($($int:ty => $uint:ty),*) => {
        $(
            impl<const N: usize> Signed for Simd<$int, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Unsigned = Simd<$uint, N>;

                #[inline]
                fn abs(self) -> Self {
                    SimdInt::abs(self)
                }
                #[inline]
                fn abs_diff(self, rhs: Self) -> Self::Unsigned {
                    SimdInt::abs_diff(self, rhs)
                }
                #[inline]
                fn signum(self) -> Self {
                    SimdInt::signum(self)
                }
                #[inline]
                fn is_positive(self) -> Self::Bool {
                    SimdInt::is_positive(self)
                }
                #[inline]
                fn is_negative(self) -> Self::Bool {
                    SimdInt::is_negative(self)
                }
            }
        )*
    };
}

impl_signed_simd!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

macro_rules! impl_signed_float_simd {
    ($($float:ty),*) => {
        $(
            impl<const N: usize> Signed for Simd<$float, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Unsigned = Self;

                #[inline]
                fn abs(self) -> Self {
                    SimdFloat::abs(self)
                }
                #[inline]
                fn abs_diff(self, rhs: Self) -> Self {
                    SimdFloat::abs(self - rhs)
                }
                #[inline]
                fn signum(self) -> Self {
                    SimdFloat::signum(self)
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

impl_signed_float_simd!(f32, f64);

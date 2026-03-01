use crate::cmp::NumOrd;
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount, num::SimdFloat};

macro_rules! impl_num_ord_simd_int {
    ($simd:ident, $($int:ty),*) => {
        $(
             impl<const N: usize> NumOrd for $simd<$int, N>
             where
                 LaneCount<N>: SupportedLaneCount,
             {
                 #[inline]
                 fn num_lt(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_lt(self, other)
                 }

                 #[inline]
                 fn num_le(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_le(self, other)
                 }

                 #[inline]
                 fn num_gt(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_gt(self, other)
                 }

                 #[inline]
                 fn num_ge(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_ge(self, other)
                 }

                 #[inline]
                 fn min(self, other: Self) -> Self {
                     core::simd::cmp::SimdOrd::simd_min(self, other)
                 }

                 #[inline]
                 fn max(self, other: Self) -> Self {
                     core::simd::cmp::SimdOrd::simd_max(self, other)
                 }

                 #[inline]
                 fn clamp(self, min: Self, max: Self) -> Self {
                     core::simd::cmp::SimdOrd::simd_clamp(self, min, max)
                 }
             }
        )*
    };
}

impl_num_ord_simd_int!(Simd, u8, u16, u32, u64, usize);
impl_num_ord_simd_int!(Simd, i8, i16, i32, i64, isize);
impl_num_ord_simd_int!(Mask, i8, i16, i32, i64, isize);

macro_rules! impl_num_ord_simd_float {
    ($simd:ident, $($float:ty),*) => {
        $(
             impl<const N: usize> NumOrd for $simd<$float, N>
             where
                 LaneCount<N>: SupportedLaneCount,
             {
                 #[inline]
                 fn num_lt(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_lt(self, other)
                 }

                 #[inline]
                 fn num_le(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_le(self, other)
                 }

                 #[inline]
                 fn num_gt(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_gt(self, other)
                 }

                 #[inline]
                 fn num_ge(self, other: Self) -> Self::Bool {
                     core::simd::cmp::SimdPartialOrd::simd_ge(self, other)
                 }

                 #[inline]
                 fn min(self, other: Self) -> Self {
                     SimdFloat::simd_min(self, other)
                 }

                 #[inline]
                 fn max(self, other: Self) -> Self {
                     SimdFloat::simd_max(self, other)
                 }

                 #[inline]
                 fn clamp(self, min: Self, max: Self) -> Self {
                     SimdFloat::simd_clamp(self, min, max)
                 }
             }
        )*
    };
}

impl_num_ord_simd_float!(Simd, f32, f64);

use crate::num::{NegOne, Num, One, Zero};
use core::simd::{LaneCount, Simd, SupportedLaneCount};

macro_rules! impl_num_simd {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> Num for Simd<$t, N> where LaneCount<N>: SupportedLaneCount {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
            }
        )*
    };
}

impl_num_simd!(u8, u16, u32, u64, usize);
impl_num_simd!(i8, i16, i32, i64, isize);
impl_num_simd!(f32, f64);

macro_rules! impl_zero_one_simd {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> Zero for Simd<$t, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                const ZERO: Self = Self::splat(0 as $t);
            }

            impl<const N: usize> One for Simd<$t, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                const ONE: Self = Self::splat(1 as $t);
            }
        )*
    };
}

impl_zero_one_simd!(u8, u16, u32, u64, usize);
impl_zero_one_simd!(i8, i16, i32, i64, isize);
impl_zero_one_simd!(f32, f64);

macro_rules! impl_neg_one_simd {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> NegOne for Simd<$t, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                const NEG_ONE: Self = Self::splat(-1 as $t);
            }
        )*
    };
}

impl_neg_one_simd!(i8, i16, i32, i64, isize);
impl_neg_one_simd!(f32, f64);

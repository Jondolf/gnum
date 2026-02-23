use super::{NegOne, One, Zero};
use core::simd::{LaneCount, Simd, SupportedLaneCount};

impl<const N: usize> Zero for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Self = Self::splat(0.0);
}

impl<const N: usize> Zero for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Self = Self::splat(0.0);
}

impl<const N: usize> One for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ONE: Self = Self::splat(1.0);
}

impl<const N: usize> One for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ONE: Self = Self::splat(1.0);
}

impl<const N: usize> NegOne for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const NEG_ONE: Self = Self::splat(-1.0);
}

impl<const N: usize> NegOne for Simd<f64, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const NEG_ONE: Self = Self::splat(-1.0);
}

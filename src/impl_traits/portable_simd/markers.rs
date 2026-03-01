use crate::num::SimdValue;
use core::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

impl<T: SimdElement, const N: usize> SimdValue for Simd<T, N> where LaneCount<N>: SupportedLaneCount {}
impl<T: MaskElement, const N: usize> SimdValue for Mask<T, N> where LaneCount<N>: SupportedLaneCount {}

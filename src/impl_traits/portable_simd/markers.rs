use crate::num::SimdValue;
use core::simd::{Mask, MaskElement, Simd, SimdElement};

impl<T: SimdElement, const N: usize> SimdValue for Simd<T, N> {}
impl<T: MaskElement, const N: usize> SimdValue for Mask<T, N> {}

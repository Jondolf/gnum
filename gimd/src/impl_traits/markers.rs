use crate::{Mask, Simd, SimdElement};
use gnum::num::SimdValue;

impl<T: SimdElement<N>, const N: usize> SimdValue for Simd<T, N> {}

impl<T: SimdElement<N>, const N: usize> SimdValue for Mask<T, N> {}

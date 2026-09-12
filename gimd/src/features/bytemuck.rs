use crate::{Simd, SimdElement};

unsafe impl<T: SimdElement<N>, const N: usize> bytemuck::Zeroable for Simd<T, N> where
    <T as SimdElement<N>>::Vector: bytemuck::Zeroable
{
}

unsafe impl<T: SimdElement<N>, const N: usize> bytemuck::Pod for Simd<T, N> where
    <T as SimdElement<N>>::Vector: bytemuck::Pod
{
}

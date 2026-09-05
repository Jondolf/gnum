use crate::num::NumCast;
use core::simd::{Simd, SimdCast, num::*};

macro_rules! impl_num_cast_simd {
    ($simd_trait:ident: $($src:ty),+ $(,)?) => {
        $(
            impl<T: SimdCast, const N: usize> NumCast<Simd<T, N>> for Simd<$src, N> {
                #[inline(always)]
                fn cast(self) -> Simd<T, N> {
                    <Self as $simd_trait>::cast::<T>(self)
                }
            }
        )+
    };
}

impl_num_cast_simd!(SimdUint: u8, u16, u32, u64, usize);
impl_num_cast_simd!(SimdInt: i8, i16, i32, i64, isize);
impl_num_cast_simd!(SimdFloat: f32, f64);

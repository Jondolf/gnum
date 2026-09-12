use crate::simd::{Reduce, ReduceBitwise, SimdLike, generic_reduce_stable};
use core::simd::{
    Simd,
    num::{SimdFloat, SimdInt, SimdUint},
};

macro_rules! impl_reduce_float {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> Reduce for Simd<$t, N>
            where
                Simd<$t, N>: SimdLike<Element = $t>,
            {
                #[inline]
                fn reduce_sum(self) -> Self::Element {
                    SimdFloat::reduce_sum(self)
                }
                #[inline]
                fn reduce_sum_stable(self) -> Self::Element {
                    generic_reduce_stable(Simd::to_array(self), |a: $t, b| a + b)
                }
                #[inline]
                fn reduce_product(self) -> Self::Element {
                    SimdFloat::reduce_product(self)
                }
                #[inline]
                fn reduce_product_stable(self) -> Self::Element {
                    generic_reduce_stable(Simd::to_array(self), |a: $t, b| a * b)
                }
                #[inline]
                fn reduce_min(self) -> Self::Element {
                    SimdFloat::reduce_min(self)
                }
                #[inline]
                fn reduce_max(self) -> Self::Element {
                    SimdFloat::reduce_max(self)
                }
            }
        )*
    };
}

macro_rules! impl_reduce_int {
    ($($t:ty => $trait:ident),*) => {
        $(
            impl<const N: usize> Reduce for Simd<$t, N>
            where
                Simd<$t, N>: SimdLike<Element = $t>,
            {
                #[inline]
                fn reduce_sum(self) -> Self::Element {
                    $trait::reduce_sum(self)
                }
                #[inline]
                fn reduce_sum_stable(self) -> Self::Element {
                    $trait::reduce_sum(self)
                }
                #[inline]
                fn reduce_product(self) -> Self::Element {
                    $trait::reduce_product(self)
                }
                #[inline]
                fn reduce_product_stable(self) -> Self::Element {
                    $trait::reduce_product(self)
                }
                #[inline]
                fn reduce_min(self) -> Self::Element {
                    $trait::reduce_min(self)
                }
                #[inline]
                fn reduce_max(self) -> Self::Element {
                    $trait::reduce_max(self)
                }
            }

            impl<const N: usize> ReduceBitwise for Simd<$t, N>
            where
                Simd<$t, N>: SimdLike<Element = $t>,
            {
                #[inline]
                fn reduce_and(self) -> Self::Element {
                    $trait::reduce_and(self)
                }
                #[inline]
                fn reduce_or(self) -> Self::Element {
                    $trait::reduce_or(self)
                }
                #[inline]
                fn reduce_xor(self) -> Self::Element {
                    $trait::reduce_xor(self)
                }
            }
        )*
    };
}

impl_reduce_float!(f32, f64);
impl_reduce_int!(
    i8 => SimdInt, i16 => SimdInt, i32 => SimdInt, i64 => SimdInt, isize => SimdInt,
    u8 => SimdUint, u16 => SimdUint, u32 => SimdUint, u64 => SimdUint, usize => SimdUint
);

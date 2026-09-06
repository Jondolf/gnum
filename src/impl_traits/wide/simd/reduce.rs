use crate::simd::{Reduce, ReduceBitwise, generic_reduce_stable};
use wide::*;

macro_rules! impl_reduce_float {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl Reduce for $simd {
                #[inline]
                fn reduce_sum(self) -> Self::Element {
                    self.reduce_add()
                }
                #[inline]
                fn reduce_product(self) -> Self::Element {
                    self.reduce_mul()
                }
                #[inline]
                fn reduce_sum_stable(self) -> Self::Element {
                    generic_reduce_stable(self.to_array(), |a: $elem, b| a + b)
                }
                #[inline]
                fn reduce_product_stable(self) -> Self::Element {
                    generic_reduce_stable(self.to_array(), |a: $elem, b| a * b)
                }
                #[inline]
                fn reduce_max(self) -> Self::Element {
                    self.to_array().into_iter().reduce(|a, b| a.max(b)).unwrap()
                }
                #[inline]
                fn reduce_min(self) -> Self::Element {
                    self.to_array().into_iter().reduce(|a, b| a.min(b)).unwrap()
                }
            }
        )*
    };
}

macro_rules! impl_reduce_int {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl Reduce for $simd {
                #[inline]
                fn reduce_sum(self) -> Self::Element {
                    self.reduce_add()
                }
                #[inline]
                fn reduce_product(self) -> Self::Element {
                    self.reduce_mul()
                }
                #[inline]
                fn reduce_sum_stable(self) -> Self::Element {
                    self.reduce_add()
                }
                #[inline]
                fn reduce_product_stable(self) -> Self::Element {
                    self.reduce_mul()
                }
                #[inline]
                fn reduce_max(self) -> Self::Element {
                    self.reduce_max()
                }
                #[inline]
                fn reduce_min(self) -> Self::Element {
                    self.reduce_min()
                }
            }

            impl ReduceBitwise for $simd {
                #[inline]
                fn reduce_and(self) -> Self::Element {
                    self.to_array().into_iter().reduce(|a, b| a & b).unwrap()
                }
                #[inline]
                fn reduce_or(self) -> Self::Element {
                    self.to_array().into_iter().reduce(|a, b| a | b).unwrap()
                }
                #[inline]
                fn reduce_xor(self) -> Self::Element {
                    self.to_array().into_iter().reduce(|a, b| a ^ b).unwrap()
                }
            }
        )*
    };
}

impl_reduce_float!(
    f32x4 => f32, f32x8 => f32, f32x16 => f32,
    f64x2 => f64, f64x4 => f64, f64x8 => f64,
);

impl_reduce_int!(
    i8x16 => i8, i8x32 => i8, i8x64 => i8,
    i16x8 => i16, i16x16 => i16, i16x32 => i16,
    i32x4 => i32, i32x8 => i32, i32x16 => i32,
    i64x2 => i64, i64x4 => i64, i64x8 => i64,
    u8x16 => u8, u8x32 => u8, u8x64 => u8,
    u16x8 => u16, u16x16 => u16, u16x32 => u16,
    u32x4 => u32, u32x8 => u32, u32x16 => u32,
    u64x2 => u64, u64x4 => u64, u64x8 => u64,
);

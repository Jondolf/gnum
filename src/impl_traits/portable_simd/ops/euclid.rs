use crate::{
    cmp::NumOrd,
    num::{
        Real, Signed, Zero,
        ops::{DivEuclid, RemEuclid},
    },
    simd::Select,
};
use core::simd::Simd;

macro_rules! impl_div_rem_euclid_int {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> DivEuclid for Simd<$t, N> {
                type Output = Self;

                #[inline]
                fn div_euclid(self, rhs: Self) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].div_euclid(rhs[i]);
                    }
                    result
                }
            }

            impl<const N: usize> RemEuclid for Simd<$t, N> {
                type Output = Self;

                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self {
                    let mut result = self;
                    for i in 0..Self::LEN {
                        result[i] = result[i].rem_euclid(rhs[i]);
                    }
                    result
                }
            }
        )*
     };
 }

impl_div_rem_euclid_int!(i8, i16, i32, i64, isize);
impl_div_rem_euclid_int!(u8, u16, u32, u64, usize);

macro_rules! impl_div_rem_euclid_real_simd {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> DivEuclid for Simd<$t, N> {
                type Output = Self;

                #[inline]
                fn div_euclid(self, rhs: Self) -> Self {
                    let q = Real::trunc(self / rhs);
                    let r = self - q * rhs;

                    let mask = r.num_lt(Self::ZERO);
                    let correction = Signed::signum(rhs);
                    mask.select(q - correction, q)
                }
            }

            impl<const N: usize> RemEuclid for Simd<$t, N> {
                type Output = Self;

                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self {
                    let r = self % rhs;
                    let mask = r.num_lt(Self::ZERO);
                    let adjusted = r + Signed::abs(rhs);
                    mask.select(adjusted, r)
                }
            }
        )*
    };
}

impl_div_rem_euclid_real_simd!(f32, f64);

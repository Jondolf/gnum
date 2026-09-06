use crate::cmp::NumOrd;
use wide::*;

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::arch::x86::*;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::arch::x86_64::*;

macro_rules! impl_comparisons {
    ($simd:ident) => {
        #[inline]
        fn num_lt(self, other: Self) -> Self::Bool {
            self.simd_lt(other)
        }

        #[inline]
        fn num_le(self, other: Self) -> Self::Bool {
            self.simd_le(other)
        }

        #[inline]
        fn num_gt(self, other: Self) -> Self::Bool {
            self.simd_gt(other)
        }

        #[inline]
        fn num_ge(self, other: Self) -> Self::Bool {
            self.simd_ge(other)
        }
    };
}

macro_rules! impl_num_ord_int {
    ($($simd:ident),* $(,)?) => {
        $(
            impl NumOrd for $simd {
                impl_comparisons!($simd);

                #[inline]
                fn min(self, other: Self) -> Self {
                    self.min(other)
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    self.max(other)
                }

                #[inline]
                fn clamp(self, min: Self, max: Self) -> Self {
                    assert!(min.simd_le(max).all(), "min > max in NumOrd::clamp");
                    self.clamp(min, max)
                }
            }
        )*
    };
}

macro_rules! impl_num_ord_float_common {
    ($simd:ident, $elem:ty) => {
        impl_comparisons!($simd);

        #[inline]
        fn min(self, other: Self) -> Self {
            zip_lanes!($simd, self, other, |a: $elem, b: $elem| a.min(b))
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            zip_lanes!($simd, self, other, |a: $elem, b: $elem| a.max(b))
        }

        #[inline]
        fn clamp(self, min: Self, max: Self) -> Self {
            assert!(min.simd_le(max).all(), "min > max in NumOrd::clamp");
            self.simd_lt(min)
                .select(min, self.simd_gt(max).select(max, self))
        }
    };
}

macro_rules! impl_num_ord_float {
    ($($simd:ident => $elem:ty),* $(,)?) => {
        $(
            impl NumOrd for $simd {
                impl_num_ord_float_common!($simd, $elem);
            }
        )*
    };
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2"
))]
macro_rules! impl_num_ord_float_x86 {
    ($($simd:ident => $elem:ty, $reg:ty, $min:ident, $max:ident);* $(;)?) => {
        $(
            impl NumOrd for $simd {
                impl_num_ord_float_common!($simd, $elem);

                #[inline]
                fn min_fast(self, other: Self) -> Self {
                    const {
                        assert!(size_of::<$simd>() == size_of::<$reg>());
                        assert!(align_of::<$simd>() >= align_of::<$reg>());
                    }

                    unsafe {
                        let a: $reg = core::mem::transmute(self);
                        let b: $reg = core::mem::transmute(other);
                        core::mem::transmute($min(a, b))
                    }
                }

                #[inline]
                fn max_fast(self, other: Self) -> Self {
                    const {
                        assert!(size_of::<$simd>() == size_of::<$reg>());
                        assert!(align_of::<$simd>() >= align_of::<$reg>());
                    }

                    unsafe {
                        let a: $reg = core::mem::transmute(self);
                        let b: $reg = core::mem::transmute(other);
                        core::mem::transmute($max(a, b))
                    }
                }
            }
        )*
    };
}

// 128-bit floats: SSE/SSE2, present on every x86_64 target
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2"
))]
impl_num_ord_float_x86! {
    f32x4 => f32, __m128, _mm_min_ps, _mm_max_ps;
    f64x2 => f64, __m128d, _mm_min_pd, _mm_max_pd;
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2"
)))]
impl_num_ord_float!(f32x4 => f32, f64x2 => f64);

// 256-bit floats: AVX
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx"
))]
impl_num_ord_float_x86! {
    f32x8 => f32, __m256, _mm256_min_ps, _mm256_max_ps;
    f64x4 => f64, __m256d, _mm256_min_pd, _mm256_max_pd;
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx"
)))]
impl_num_ord_float!(f32x8 => f32, f64x4 => f64);

// AVX-512 widths always use the portable path
impl_num_ord_float!(f32x16 => f32, f64x8 => f64);

impl_num_ord_int!(
    i8x16, i8x32, i8x64, i16x8, i16x16, i16x32, i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16,
    u8x32, u8x64, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
);

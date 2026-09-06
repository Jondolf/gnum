use crate::simd::{Shuffle4, Shuffle8};
use wide::*;

macro_rules! impl_shuffle4 {
    ($($simd:ident),* $(,)?) => {
        $(
            impl Shuffle4 for $simd {
                #[inline(always)]
                fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(
                    self,
                ) -> Self {
                    const {
                        assert!(
                            A < 4 && B < 4 && C < 4 && D < 4,
                            "shuffle index out of bounds: 4-lane vectors have lanes 0..=3",
                        );
                    }

                    let a = self.to_array();
                    <$simd>::new([a[A], a[B], a[C], a[D]])
                }
            }
        )*
    };
}

macro_rules! impl_shuffle8 {
    ($($simd:ident),* $(,)?) => {
        $(
            impl Shuffle8 for $simd {
                #[inline(always)]
                fn shuffle<
                    const A: usize,
                    const B: usize,
                    const C: usize,
                    const D: usize,
                    const E: usize,
                    const F: usize,
                    const G: usize,
                    const H: usize,
                >(
                    self,
                ) -> Self {
                    const {
                        assert!(
                            A < 8 && B < 8 && C < 8 && D < 8 && E < 8 && F < 8 && G < 8 && H < 8,
                            "shuffle index out of bounds: 8-lane vectors have lanes 0..=7",
                        );
                    }

                    let a = self.to_array();
                    <$simd>::new([a[A], a[B], a[C], a[D], a[E], a[F], a[G], a[H]])
                }
            }
        )*
    };
}

impl_shuffle4!(f32x4, f64x4, i32x4, u32x4, i64x4, u64x4);
impl_shuffle8!(f32x8, f64x8, i32x8, u32x8, i64x8, u64x8);

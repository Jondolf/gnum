use super::SimdLike;

/// Lane shuffling for 4-lane SIMD types.
///
/// Shuffles are the primitive behind lane rotation, reversal, interleaving, and horizontal
/// reductions. They are also used for many optimized mathematical routines and algorithms.
///
/// Here, the lane indices are provided as const parameters rather than arguments.
/// This allows the compiler to translate the shuffle into a single machine instruction.
///
/// For 8-lane SIMD types, see [`Shuffle8`].
///
/// # Examples
///
/// ```
/// #![feature(portable_simd)]
/// # #[cfg(feature = "portable_simd")]
/// # {
/// use core::simd::f32x4;
/// use gnum::simd::{Shuffle4, SimdLike, Swizzle};
///
/// let v = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
///
/// // Broadcast lane 0 into every lane.
/// let broadcast = Shuffle4::shuffle::<0, 0, 0, 0>(v);
/// assert_eq!(SimdLike::to_array(broadcast), [1.0, 1.0, 1.0, 1.0]);
///
/// // Rotate lanes: `[x, y, z, w]` -> `[y, z, w, x]`.
/// let rotated = Shuffle4::shuffle::<1, 2, 3, 0>(v);
/// assert_eq!(SimdLike::to_array(rotated), [2.0, 3.0, 4.0, 1.0]);
///
/// // Equivalently, we can use the `rotate_elements_left` method from `Swizzle`.
/// let rotated2 = Swizzle::rotate_elements_left::<1>(v);
/// assert_eq!(SimdLike::to_array(rotated2), [2.0, 3.0, 4.0, 1.0]);
/// # }
/// ```
pub trait Shuffle4: SimdLike {
    /// Returns a vector whose lanes are `[self[A], self[B], self[C], self[D]]`.
    ///
    /// Each index must be less than `4`, or the call fails to compile.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(self) -> Self;
}

/// Lane shuffling for 8-lane SIMD types.
///
/// Shuffles are the primitive behind lane rotation, reversal, interleaving, and horizontal
/// reductions. They are also used for many optimized mathematical routines and algorithms.
///
/// Here, the lane indices are provided as const parameters rather than arguments.
/// This allows the compiler to translate the shuffle into a single machine instruction.
///
/// For 4-lane SIMD types, see [`Shuffle4`].
///
/// # Examples
///
/// ```
/// #![feature(portable_simd)]
/// # #[cfg(feature = "portable_simd")]
/// # {
/// use core::simd::f32x8;
/// use gnum::simd::{Shuffle8, SimdLike, Swizzle};
///
/// let v = f32x8::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
///
/// // Broadcast lane 0 into every lane.
/// let broadcast = Shuffle8::shuffle::<0, 0, 0, 0, 0, 0, 0, 0>(v);
/// assert_eq!(SimdLike::to_array(broadcast), [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
///
/// // Rotate lanes: `[x, y, z, w, a, b, c, d]` -> `[y, z, w, a, b, c, d, x]`.
/// let rotated = Shuffle8::shuffle::<1, 2, 3, 4, 5, 6, 7, 0>(v);
/// assert_eq!(SimdLike::to_array(rotated), [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0]);
///
/// // Equivalently, we can use the `rotate_elements_left` method from `Swizzle`.
/// let rotated2 = Swizzle::rotate_elements_left::<1>(v);
/// assert_eq!(SimdLike::to_array(rotated2), [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0]);
/// # }
/// ```
pub trait Shuffle8: SimdLike {
    /// Returns a vector whose lanes are `[self[A], self[B], ..., self[H]]`.
    ///
    /// Each index must be less than `8`, or the call fails to compile.
    #[must_use = "method returns a new vector and does not mutate the original value"]
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
    ) -> Self;
}

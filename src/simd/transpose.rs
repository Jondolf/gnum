#[cfg(feature = "portable_simd")]
use super::Swizzle;

/// Transposes a square matrix represented by an array of SIMD vectors.
///
/// # Examples
///
/// ```
/// #![feature(portable_simd)]
/// # #[cfg(feature = "portable_simd")]
/// # {
/// use core::simd::i32x4;
/// use gnum::simd::Transpose;
///
/// let rows = [
///     i32x4::from_array([0, 1, 2, 3]),
///     i32x4::from_array([4, 5, 6, 7]),
///     i32x4::from_array([8, 9, 10, 11]),
///     i32x4::from_array([12, 13, 14, 15]),
/// ];
/// let columns = rows.transpose();
/// assert_eq!(columns[0].to_array(), [0, 4, 8, 12]);
/// # }
/// ```
pub trait Transpose: Sized {
    /// Returns the transpose of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(portable_simd)]
    /// # #[cfg(feature = "portable_simd")]
    /// # {
    /// use core::simd::i32x4;
    /// use gnum::simd::Transpose;
    ///
    /// let rows = [
    ///     i32x4::from_array([0, 1, 2, 3]),
    ///     i32x4::from_array([4, 5, 6, 7]),
    ///     i32x4::from_array([8, 9, 10, 11]),
    ///     i32x4::from_array([12, 13, 14, 15]),
    /// ];
    /// let columns = rows.transpose();
    /// assert_eq!(columns[0].to_array(), [0, 4, 8, 12]);
    /// # }
    /// ```
    #[must_use = "transpose returns a new matrix and does not mutate the input"]
    fn transpose(self) -> Self;
}

#[inline]
#[cfg(feature = "portable_simd")]
pub(crate) fn generic_transpose<T: Swizzle + Copy, const N: usize>(rows: [T; N]) -> [T; N] {
    let half = N / 2;
    let mut current = rows;

    let mut remaining = N;
    while remaining > 1 {
        let mut next = current;
        let mut i = 0;
        while i < half {
            let (lo, hi) = current[i].interleave(current[i + half]);
            next[2 * i] = lo;
            next[2 * i + 1] = hi;
            i += 1;
        }
        current = next;
        remaining >>= 1;
    }

    current
}

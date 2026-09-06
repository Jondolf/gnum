#[cfg(any(feature = "portable_simd", feature = "wide"))]
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

/// Transposes an `N`x`N` matrix by splitting each row into `K` blocks of `BN` lanes,
/// transposing each `BN`x`BN` block, and writing the blocks back transposed.
#[inline(always)]
#[cfg(any(feature = "portable_simd", feature = "wide"))]
pub(crate) fn blocked_transpose<T, B, const N: usize, const BN: usize, const K: usize>(
    rows: [T; N],
    split: impl Fn(T) -> [B; K],
    join: impl Fn([B; K]) -> T,
    transpose_block: impl Fn([B; BN]) -> [B; BN],
) -> [T; N]
where
    T: Copy,
    B: Copy,
{
    let row_blocks = rows.map(split);
    let mut column_blocks = [[row_blocks[0][0]; K]; N];

    let mut block_row = 0;
    while block_row < K {
        let mut block_column = 0;
        while block_column < K {
            let block = core::array::from_fn(|r| row_blocks[block_row * BN + r][block_column]);
            let block = transpose_block(block);
            let mut c = 0;
            while c < BN {
                column_blocks[block_column * BN + c][block_row] = block[c];
                c += 1;
            }
            block_column += 1;
        }
        block_row += 1;
    }

    column_blocks.map(join)
}

#[inline]
#[cfg(any(feature = "portable_simd", feature = "wide"))]
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

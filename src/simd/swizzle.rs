use super::SimdLike;

/// Lane permutations for SIMD types.
///
/// Where [`Shuffle4`] and [`Shuffle8`] take an explicit list of source lanes,
/// this trait provides the common permutations built on top of that primitive:
/// reversing, rotating, shifting, and interleaving.
///
/// [`Shuffle4`]: crate::simd::Shuffle4
/// [`Shuffle8`]: crate::simd::Shuffle8
///
/// # Examples
///
/// ```
/// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
///
/// let a = i32x4([0, 1, 2, 3]);
/// assert_eq!(a.reverse().to_array(), [3, 2, 1, 0]);
/// ```
pub trait Swizzle: SimdLike {
    /// Reverses the order of the elements in the vector.
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// assert_eq!(a.reverse().to_array(), [3, 2, 1, 0]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn reverse(self) -> Self;

    /// Rotates the vector such that the first `OFFSET` elements of the slice move
    /// to the end while the last `self.len() - OFFSET` elements move to the front.
    ///
    /// After calling `rotate_elements_left`, the element previously at index `OFFSET`
    /// will become the first element in the slice.
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// let x = a.rotate_elements_left::<3>();
    /// assert_eq!(x.to_array(), [3, 0, 1, 2]);
    ///
    /// // The offset wraps around for offsets larger than the lane count.
    /// let y = a.rotate_elements_left::<7>();
    /// assert_eq!(y.to_array(), [3, 0, 1, 2]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn rotate_elements_left<const OFFSET: usize>(self) -> Self;

    /// Rotates the vector such that the first `self.len() - OFFSET` elements of the vector
    /// move to the end while the last `OFFSET` elements move to the front.
    ///
    /// After calling `rotate_elements_right`, the element previously at index
    /// `self.len() - OFFSET` will become the first element in the slice.
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// let x = a.rotate_elements_right::<3>();
    /// assert_eq!(x.to_array(), [1, 2, 3, 0]);
    ///
    /// // The offset wraps around for offsets larger than the lane count.
    /// let y = a.rotate_elements_right::<7>();
    /// assert_eq!(y.to_array(), [1, 2, 3, 0]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn rotate_elements_right<const OFFSET: usize>(self) -> Self;

    /// Shifts the vector elements to the left by `OFFSET`, filling in with
    /// `padding` from the right.
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// let x = a.shift_elements_left::<3>(255);
    /// assert_eq!(x.to_array(), [3, 255, 255, 255]);
    ///
    /// let y = a.shift_elements_left::<7>(255);
    /// assert_eq!(y.to_array(), [255, 255, 255, 255]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn shift_elements_left<const OFFSET: usize>(self, padding: Self::Element) -> Self;

    /// Shifts the vector elements to the right by `OFFSET`, filling in with
    /// `padding` from the left.
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// let x = a.shift_elements_right::<3>(255);
    /// assert_eq!(x.to_array(), [255, 255, 255, 0]);
    ///
    /// let y = a.shift_elements_right::<7>(255);
    /// assert_eq!(y.to_array(), [255, 255, 255, 255]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn shift_elements_right<const OFFSET: usize>(self, padding: Self::Element) -> Self;

    /// Interleaves two vectors.
    ///
    /// The resulting vectors contain elements alternating between `self` and `other`,
    /// first filling the first result, and then the second.
    ///
    /// The reverse of this operation is [`Swizzle::deinterleave`].
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 1, 2, 3]);
    /// let b = i32x4([4, 5, 6, 7]);
    /// let (x, y) = a.interleave(b);
    /// assert_eq!(x.to_array(), [0, 4, 1, 5]);
    /// assert_eq!(y.to_array(), [2, 6, 3, 7]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn interleave(self, other: Self) -> (Self, Self);

    /// Deinterleaves two vectors.
    ///
    /// The first result takes every other element of `self` and then `other`,
    /// starting with the first element.
    ///
    /// The second result takes every other element of `self` and then `other`,
    /// starting with the second element.
    ///
    /// The reverse of this operation is [`Swizzle::interleave`].
    ///
    /// ```
    /// use gnum::{i32x4, simd::{SimdLike, Swizzle}};
    ///
    /// let a = i32x4([0, 4, 1, 5]);
    /// let b = i32x4([2, 6, 3, 7]);
    /// let (x, y) = a.deinterleave(b);
    /// assert_eq!(x.to_array(), [0, 1, 2, 3]);
    /// assert_eq!(y.to_array(), [4, 5, 6, 7]);
    /// ```
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn deinterleave(self, other: Self) -> (Self, Self);
}

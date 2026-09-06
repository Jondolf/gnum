use super::SimdLike;

/// Horizontal reductions that collapse the lanes of a SIMD vector into a single scalar.
///
/// For a scalar, which has a single lane, every reduction returns the value unchanged,
/// so generic SIMD code can reduce over a scalar or a vector uniformly.
///
/// See [`ReduceBitwise`] for bitwise reductions, which only apply to integers.
///
/// # Example
///
/// ```
/// use gnum::{f32x4, simd::Reduce};
///
/// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(v.reduce_sum(), 10.0);
/// assert_eq!(v.reduce_max(), 4.0);
/// ```
pub trait Reduce: SimdLike {
    /// Returns the sum of the lanes of the vector.
    ///
    /// For integers, this uses wrapping addition.
    ///
    /// # Unspecified Order
    ///
    /// The *order* in which the lanes are summed is backend-defined.
    /// For some non-associative types (ex: floating-point numbers),
    /// this can result in different results across backends or compilation targets.
    ///
    /// See [`reduce_sum_stable`](Self::reduce_sum_stable) for a version of this function
    /// that is commonly faster and more accurate, and guaranteed to be deterministic
    /// and return identical results across backends.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_sum(), 10.0);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_sum(self) -> Self::Element;

    /// Returns the sum of the lanes of the vector.
    ///
    /// For integers, this uses wrapping addition.
    ///
    /// # Order
    ///
    /// The lanes are summed as a balanced binary tree, pairing each lane
    /// with the one half a vector away and halving the width each round.
    ///
    /// For four lanes, this is `(v[0] + v[2]) + (v[1] + v[3])`, and for eight lanes,
    /// this is `((v[0] + v[4]) + (v[2] + v[6])) + ((v[1] + v[5]) + (v[3] + v[7]))`.
    ///
    /// This is commonly both faster and more accurate than a sequential sum,
    /// and is guaranteed to be deterministic and return identical results across backends.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_sum_stable(), 10.0);
    ///
    /// // The above is bit-identical to the following
    /// let sum = (v[0] + v[2]) + (v[1] + v[3]);
    /// assert_eq!(v.reduce_sum_stable(), sum);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_sum_stable(self) -> Self::Element;

    /// Returns the product of the lanes of the vector.
    ///
    /// For integers, this uses wrapping multiplication.
    ///
    /// # Unspecified Order
    ///
    /// The *order* in which the lanes are multiplied is backend-defined.
    /// For some non-associative types (ex: floating-point numbers),
    /// this can result in different results across backends or compilation targets.
    ///
    /// See [`reduce_product_stable`](Self::reduce_product_stable) for a version of this function
    /// that is commonly faster and more accurate, and guaranteed to be deterministic
    /// and return identical results across backends.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_product(), 24.0);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_product(self) -> Self::Element;

    /// Returns the product of the lanes of the vector.
    ///
    /// For integers, this uses wrapping multiplication.
    ///
    /// # Order
    ///
    /// The lanes are multiplied as a balanced binary tree, pairing each lane
    /// with the one half a vector away and halving the width each round.
    ///
    /// For four lanes, this is `(v[0] * v[2]) * (v[1] * v[3])`, and for eight lanes,
    /// this is `((v[0] * v[4]) * (v[2] * v[6])) * ((v[1] * v[5]) * (v[3] * v[7]))`.
    ///
    /// This is commonly both faster and more accurate than a sequential product,
    /// and is guaranteed to be deterministic and return identical results across backends.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_product_stable(), 24.0);
    ///
    /// // The above is bit-identical to the following
    /// let product = (v[0] * v[2]) * (v[1] * v[3]);
    /// assert_eq!(v.reduce_product_stable(), product);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_product_stable(self) -> Self::Element;

    /// Returns the minimum lane of the vector.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, each comparison between lanes follows the semantics of [`f32::min`].
    ///
    /// - If exactly one of the values is NaN (quiet or signaling), then the other value is returned.
    /// - If both values are NaN, the return value is NaN, with the bit pattern picked using the usual rules for arithmetic operations.
    /// - If the values compare equal (such as for the case of `+0.0` and `-0.0`), either value may be returned non-deterministically.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_min(), 1.0);
    ///
    /// let v_with_nan = f32x4([1.0, f32::NAN, 3.0, 4.0]);
    /// assert_eq!(v_with_nan.reduce_min(), 1.0);
    ///
    /// let v_with_all_nan = f32x4([f32::NAN, f32::NAN, f32::NAN, f32::NAN]);
    /// assert!(v_with_all_nan.reduce_min().is_nan());
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_min(self) -> Self::Element;

    /// Returns the maximum lane of the vector.
    ///
    /// # Floating-Point Types
    ///
    /// For floating-point types, each comparison between lanes follows the semantics of [`f32::max`].
    ///
    /// - If exactly one of the values is NaN (quiet or signaling), then the other value is returned.
    /// - If both values are NaN, the return value is NaN, with the bit pattern picked using the usual rules for arithmetic operations.
    /// - If the values compare equal (such as for the case of `+0.0` and `-0.0`), either value may be returned non-deterministically.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{f32x4, simd::Reduce};
    ///
    /// let v = f32x4([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.reduce_max(), 4.0);
    ///
    /// let v_with_nan = f32x4([1.0, f32::NAN, 3.0, 4.0]);
    /// assert_eq!(v_with_nan.reduce_max(), 4.0);
    ///
    /// let v_with_all_nan = f32x4([f32::NAN, f32::NAN, f32::NAN, f32::NAN]);
    /// assert!(v_with_all_nan.reduce_max().is_nan());
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_max(self) -> Self::Element;
}

/// Bitwise horizontal reductions for integer SIMD values.
///
/// This is the bitwise counterpart of [`Reduce`].
///
/// # Example
///
/// ```
/// use gnum::{u32x4, simd::ReduceBitwise};
///
/// let v = u32x4([0b0110, 0b0111, 0b1110, 0b0100]);
/// assert_eq!(v.reduce_and(), 0b0100);
/// assert_eq!(v.reduce_or(), 0b1111);
/// ```
pub trait ReduceBitwise: SimdLike {
    /// Returns the cumulative bitwise "and" across the lanes of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{u32x4, simd::ReduceBitwise};
    ///
    /// let v = u32x4([0b0110, 0b0111, 0b1110, 0b0100]);
    /// assert_eq!(v.reduce_and(), 0b0100);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_and(self) -> Self::Element;

    /// Returns the cumulative bitwise "or" across the lanes of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{u32x4, simd::ReduceBitwise};
    ///
    /// let v = u32x4([0b0110, 0b0111, 0b1110, 0b0100]);
    /// assert_eq!(v.reduce_or(), 0b1111);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_or(self) -> Self::Element;

    /// Returns the cumulative bitwise "xor" across the lanes of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::{u32x4, simd::ReduceBitwise};
    ///
    /// let v = u32x4([0b0110, 0b0111, 0b1110, 0b0100]);
    /// assert_eq!(v.reduce_xor(), 0b1011);
    /// ```
    #[must_use = "method returns a new element and does not mutate the original value"]
    fn reduce_xor(self) -> Self::Element;
}

#[inline]
#[cfg(any(feature = "portable_simd", feature = "wide"))]
pub(crate) fn generic_reduce_stable<T: Copy, const N: usize>(
    array: [T; N],
    f: impl Fn(T, T) -> T,
) -> T {
    let mut current = array;
    let mut len = N;

    while len > 1 {
        let half = len / 2;
        let rest = len - half;

        let mut i = 0;
        while i < half {
            current[i] = f(current[i], current[i + rest]);
            i += 1;
        }

        len = rest;
    }

    current[0]
}

macro_rules! impl_reduce_scalar {
    ($($t:ty),*) => {
        $(
            impl Reduce for $t {
                #[inline]
                fn reduce_sum(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_sum_stable(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_product(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_product_stable(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_max(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_min(self) -> Self::Element {
                    self
                }
            }
        )*
    };
}

macro_rules! impl_reduce_bitwise_scalar {
    ($($t:ty),*) => {
        $(
            impl ReduceBitwise for $t {
                #[inline]
                fn reduce_and(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_or(self) -> Self::Element {
                    self
                }
                #[inline]
                fn reduce_xor(self) -> Self::Element {
                    self
                }
            }
        )*
    };
}

impl_reduce_scalar!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
impl_reduce_bitwise_scalar!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

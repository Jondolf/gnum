use crate::num::Num;

/// Trait for casting between numeric types with the same semantics as the [`as`] operator.
///
/// For SIMD types, the cast is performed lane-wise, and is only implemented between
/// vector types with the same number of lanes.
///
/// [`as`]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
///
/// # Semantics
///
/// Casting follows the rules of the `as` operator:
///
/// - Casting from a larger integer to a smaller integer truncate the value.
/// - Casting from a smaller integer to a larger integer zero-extends or sign-extends the bit pattern.
/// - Casting from a float to an integer rounds towards zero, saturates at the
///   integer's bounds, and maps `NaN` to zero.
/// - Casting from an integer to a float rounds to the nearest representable value,
///   with ties rounding to even.
/// - Casting between floats rounds to the nearest representable value,
///   with overflowing values becoming infinite.
///
/// # Example
///
/// ```
/// use gnum::num::NumCast;
///
/// let x: f32 = 3.7;
/// let y: i32 = NumCast::cast(x);
/// assert_eq!(y, 3);
///
/// // Out-of-range floats saturate, just like `as`.
/// assert_eq!(NumCast::<u8>::cast(-1.0f32), 0);
/// assert_eq!(NumCast::<u8>::cast(300.0f32), 255);
/// ```
pub trait NumCast<T>: Num {
    /// Casts `self` to the type `T`, with the same semantics as the [`as`] operator.
    ///
    /// See the [trait documentation](NumCast) for the exact conversion rules.
    ///
    /// [`as`]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
    ///
    /// # Example
    ///
    /// ```
    /// use gnum::num::NumCast;
    ///
    /// let x: u32 = 7;
    /// let y: f64 = NumCast::cast(x);
    /// assert_eq!(y, 7.0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast(self) -> T;
}

/// Implements [`NumCast`] between every pair of the given scalar types.
macro_rules! impl_num_cast_scalar {
    ($($t:ty),+ $(,)?) => {
        impl_num_cast_scalar!(@src [$($t),+] ; $($t),+);
    };
    (@src $list:tt ; $($src:ty),+) => {
        $(impl_num_cast_scalar!(@dst $src ; $list);)+
    };
    (@dst $src:ty ; [$($dst:ty),+]) => {
        $(
            impl NumCast<$dst> for $src {
                #[inline(always)]
                fn cast(self) -> $dst {
                    self as $dst
                }
            }
        )+
    };
}

impl_num_cast_scalar!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);

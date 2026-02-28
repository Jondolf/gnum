use crate::{simd::NumOrd, traits::Num};
use core::ops::*;

/// A trait for integer types such as [`i32`] and [`u64`].
pub trait Int:
    Num
    + NumOrd
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOrAssign<Self>
    + BitXorAssign<Self>
    + ShlAssign<Self>
    + ShrAssign<Self>
{
    /// The unsigned integer type corresponding to this integer type.
    type Unsigned: Int;

    /// The size of this integer type in bits.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(i32::BITS, 32);
    /// assert_eq!(u64::BITS, 64);
    /// ```
    const BITS: u32;

    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.count_ones(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_ones(self) -> Self::Unsigned;

    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.count_zeros(), 5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_zeros(self) -> Self::Unsigned;

    /// Returns the number of leading zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.leading_zeros(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_zeros(self) -> Self::Unsigned;

    /// Returns the number of trailing zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.trailing_zeros(), 0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_zeros(self) -> Self::Unsigned;

    /// Returns the number of leading ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b11010000;
    /// assert_eq!(n.leading_ones(), 2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_ones(self) -> Self::Unsigned;

    /// Returns the number of trailing ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.trailing_ones(), 1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_ones(self) -> Self::Unsigned;

    /// Shifts the bits to the left by `n` positions, wrapping the truncated bits to the end of the resulting integer.
    ///
    /// `rotate_left(n)` is equivalent to applying `rotate_left(1)` a total of `n` times.
    /// In particular, a rotation by the number of bits in `self` returns the input value unchanged.
    ///
    /// Please note that this is *not* the same as the [`<<`](Shl) operator, which performs
    /// a logical left shift and fills the vacated bits with zeros.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.rotate_left(2), 0b11000010);
    /// assert_eq!(n.rotate_left(8), n);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_left(self, n: Self::Unsigned) -> Self;

    /// Shifts the bits to the right by `n` positions, wrapping the truncated bits to the beginning of the resulting integer.
    ///
    /// `rotate_right(n)` is equivalent to applying `rotate_right(1)` a total of `n` times.
    /// In particular, a rotation by the number of bits in `self` returns the input value unchanged.
    ///
    /// Please note that this is *not* the same as the [`>>`](Shr) operator, which performs
    /// a logical right shift and fills the vacated bits with zeros.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00110001;
    /// assert_eq!(n.rotate_right(2), 0b01001100);
    /// assert_eq!(n.rotate_right(8), n);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_right(self, n: Self::Unsigned) -> Self;

    /// Reverses the byte order of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u32 = 0x12345678;
    /// assert_eq!(n.swap_bytes(), 0x78563412);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn swap_bytes(self) -> Self;

    /// Reverses the bit order of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u8 = 0b00010010;
    /// assert_eq!(n.reverse_bits(), 0b01001000);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn reverse_bits(self) -> Self;

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian, this is a no-op. On little endian, this swaps the byte order.
    ///
    /// # Example
    /// ```
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.from_be(), n);
    /// } else {
    ///     assert_eq!(n.from_be(), n.swap_bytes());
    /// }
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_be(self) -> Self;

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian, this is a no-op. On big endian, this swaps the byte order.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.from_le(), n);
    /// } else {
    ///     assert_eq!(n.from_le(), n.swap_bytes());
    /// }
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_le(self) -> Self;

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian, this is a no-op. On little endian, this swaps the byte order.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.to_be(), n);
    /// } else {
    ///     assert_eq!(n.to_be(), n.swap_bytes());
    /// }
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_be(self) -> Self;

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian, this is a no-op. On big endian, this swaps the byte order.
    ///
    /// # Example
    ///
    /// ```
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.to_le(), n);
    /// } else {
    ///     assert_eq!(n.to_le(), n.swap_bytes());
    /// }
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_le(self) -> Self;

    /// Returns the integer logarithm of `self` with respect to `base`, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative, or if `base` is less than 2.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.ilog(2), 4);
    /// assert_eq!(15.ilog(2), 3);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// let _ = 0.ilog(2);
    /// ```
    ///
    /// The following panics because `base` is less than 2:
    ///
    /// ```should_panic
    /// let _ = 16.ilog(1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog(self, base: Self) -> Self::Unsigned;

    /// Returns the base 2 integer logarithm of `self`, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.ilog2(), 4);
    /// assert_eq!(15.ilog2(), 3);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// let _ = 0.ilog2();
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog2(self) -> Self::Unsigned;

    /// Returns the base 10 integer logarithm of `self`, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(100.ilog10(), 2);
    /// assert_eq!(99.ilog10(), 1);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// let _ = 0.ilog10();
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog10(self) -> Self::Unsigned;

    /// Returns `self` raised to the power of `exp`.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(2.pow(3), 8);
    /// assert_eq!(5.pow(0), 1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn pow(self, exp: Self::Unsigned) -> Self;

    /// Returns the integer square root of `self`, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if `self` is negative.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(16.isqrt(), 4);
    /// assert_eq!(15.isqrt(), 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn isqrt(self) -> Self;

    /// Calculates the midpoint (average) between `self` and `rhs`.
    ///
    /// `midpoint(a, b)` is `(a + b) / 2` as if it were performed in a
    /// sufficiently-large unsigned integral type. This implies that the result is
    /// always rounded towards zero and that no overflow will ever occur.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(10u16.midpoint(20u16), 15);
    /// assert_eq!(i32::MIN.midpoint(i32::MAX), -1);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[doc(alias = "average_floor")]
    #[doc(alias = "average")]
    #[inline]
    fn midpoint(self, rhs: Self) -> Self {
        // From Rust core
        // Use the well known branchless algorithm from Hacker's Delight to compute
        // `(a + b) / 2` without overflowing: `((a ^ b) >> 1) + (a & b)`.
        ((self ^ rhs) >> Self::ONE) + (self & rhs)
    }
}

macro_rules! impl_int {
    ($($int:ty => $uint:ty),*) => {
        $(
            impl Int for $int {
                type Unsigned = $uint;

                const BITS: u32 = <$int>::BITS;

                #[inline]
                fn count_ones(self) -> Self::Unsigned {
                    self.count_ones() as $uint
                }
                #[inline]
                fn count_zeros(self) -> Self::Unsigned {
                    self.count_zeros() as $uint
                }
                #[inline]
                fn leading_zeros(self) -> Self::Unsigned {
                    self.leading_zeros() as $uint
                }
                #[inline]
                fn trailing_zeros(self) -> Self::Unsigned {
                    self.trailing_zeros() as $uint
                }
                #[inline]
                fn leading_ones(self) -> Self::Unsigned {
                    self.leading_ones() as $uint
                }
                #[inline]
                fn trailing_ones(self) -> Self::Unsigned {
                    self.trailing_ones() as $uint
                }
                #[inline]
                fn rotate_left(self, n: Self::Unsigned) -> Self {
                    self.rotate_left(n as u32)
                }
                #[inline]
                fn rotate_right(self, n: Self::Unsigned) -> Self {
                    self.rotate_right(n as u32)
                }
                #[inline]
                fn swap_bytes(self) -> Self {
                    self.swap_bytes()
                }
                #[inline]
                fn reverse_bits(self) -> Self {
                    self.reverse_bits()
                }
                #[inline]
                fn from_be(self) -> Self {
                    <$int>::from_be(self)
                }
                #[inline]
                fn from_le(self) -> Self {
                    <$int>::from_le(self)
                }
                #[inline]
                fn to_be(self) -> Self {
                    self.to_be()
                }
                #[inline]
                fn to_le(self) -> Self {
                    self.to_le()
                }
                #[inline]
                fn ilog(self, base: Self) -> Self::Unsigned {
                    self.ilog(base) as $uint
                }
                #[inline]
                fn ilog2(self) -> Self::Unsigned {
                    self.ilog2() as $uint
                }
                #[inline]
                fn ilog10(self) -> Self::Unsigned {
                    self.ilog10() as $uint
                }
                #[inline]
                fn pow(self, exp: Self::Unsigned) -> Self {
                    self.pow(exp as u32)
                }
                #[inline]
                fn isqrt(self) -> Self {
                    self.isqrt()
                }
            }
        )*
    };
}

impl_int!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);
impl_int!(u8 => u8, u16 => u16, u32 => u32, u64 => u64, usize => usize);

use crate::{
    cmp::NumOrd,
    num::{Num, Signed},
};
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
    type Unsigned: Int<Signed = Self::Signed>;

    /// The signed integer type corresponding to this integer type.
    type Signed: Int<Unsigned = Self::Unsigned> + Signed;

    /// The size of this integer type in bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// assert_eq!(<i32 as Int>::BITS, 32);
    /// assert_eq!(<u64 as Int>::BITS, 64);
    /// ```
    const BITS: u32;

    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::count_ones(n), 3);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_ones(self) -> Self::Unsigned;

    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::count_zeros(n), 5);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn count_zeros(self) -> Self::Unsigned;

    /// Returns the number of leading zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::leading_zeros(n), 2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_zeros(self) -> Self::Unsigned;

    /// Returns the number of trailing zeros in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::trailing_zeros(n), 0);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn trailing_zeros(self) -> Self::Unsigned;

    /// Returns the number of leading ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b11010000;
    /// assert_eq!(Int::leading_ones(n), 2);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn leading_ones(self) -> Self::Unsigned;

    /// Returns the number of trailing ones in the binary representation of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::trailing_ones(n), 1);
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
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::rotate_left(n, 2), 0b11000100);
    /// assert_eq!(Int::rotate_left(n, 8), n);
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
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00110001;
    /// assert_eq!(Int::rotate_right(n, 2), 0b01001100);
    /// assert_eq!(Int::rotate_right(n, 8), n);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rotate_right(self, n: Self::Unsigned) -> Self;

    /// Reverses the byte order of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u32 = 0x12345678;
    /// assert_eq!(Int::swap_bytes(n), 0x78563412);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn swap_bytes(self) -> Self;

    /// Reverses the bit order of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: u8 = 0b00010010;
    /// assert_eq!(Int::reverse_bits(n), 0b01001000);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn reverse_bits(self) -> Self;

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian, this is a no-op. On little endian, this swaps the byte order.
    ///
    /// # Example
    /// ```
    /// # use gnum::num::Int;
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(Int::from_be(n), n);
    /// } else {
    ///     assert_eq!(Int::from_be(n), Int::swap_bytes(n));
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
    /// # use gnum::num::Int;
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Int::from_le(n), n);
    /// } else {
    ///     assert_eq!(Int::from_le(n), Int::swap_bytes(n));
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
    /// # use gnum::num::Int;
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(Int::to_be(n), n);
    /// } else {
    ///     assert_eq!(Int::to_be(n), Int::swap_bytes(n));
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
    /// # use gnum::num::Int;
    /// let n: u32 = 0x12345678;
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Int::to_le(n), n);
    /// } else {
    ///     assert_eq!(Int::to_le(n), Int::swap_bytes(n));
    /// }
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_le(self) -> Self;

    /// Returns the bit pattern of `self` reinterpreted as an [unsigned](Self::Unsigned)
    /// integer of the same size.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n: i32 = -1;
    /// assert_eq!(Int::cast_unsigned(n), u32::MAX);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_unsigned(self) -> Self::Unsigned;

    /// Returns the bit pattern of `self` reinterpreted as a [signed](Self::Signed)
    /// integer of the same size.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// let n = u32::MAX;
    /// assert_eq!(Int::cast_signed(n), -1i32);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_signed(self) -> Self::Signed;

    /// Returns the integer logarithm of `self` with respect to `base`, rounded down.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative, or if `base` is less than 2.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// assert_eq!(Int::ilog(16i32, 2), 4u32);
    /// assert_eq!(Int::ilog(15i32, 2), 3u32);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// # use gnum::num::Int;
    /// let _ = Int::ilog(0i32, 2);
    /// ```
    ///
    /// The following panics because `base` is less than 2:
    ///
    /// ```should_panic
    /// # use gnum::num::Int;
    /// let _ = Int::ilog(16i32, 1);
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
    /// # use gnum::num::Int;
    /// assert_eq!(Int::ilog2(16i32), 4u32);
    /// assert_eq!(Int::ilog2(15i32), 3u32);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// # use gnum::num::Int;
    /// let _ = Int::ilog2(0i32);
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
    /// # use gnum::num::Int;
    /// assert_eq!(Int::ilog10(100i32), 2u32);
    /// assert_eq!(Int::ilog10(99i32), 1u32);
    /// ```
    ///
    /// The following panics because `self` is not greater than zero:
    ///
    /// ```should_panic
    /// # use gnum::num::Int;
    /// let _ = Int::ilog10(0i32);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog10(self) -> Self::Unsigned;

    /// Returns `self` raised to the power of `exp`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gnum::num::Int;
    /// assert_eq!(Int::pow(2i32, 3u32), 8);
    /// assert_eq!(Int::pow(5i32, 0u32), 1);
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
    /// # use gnum::num::Int;
    /// assert_eq!(Int::isqrt(16), 4);
    /// assert_eq!(Int::isqrt(15), 3);
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
    /// # use gnum::num::Int;
    /// assert_eq!(Int::midpoint(10u16, 20u16), 15);
    /// assert_eq!(Int::midpoint(i32::MIN, i32::MAX), -1);
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
    ($($int:ty => $uint:ty, $sint:ty),*) => {
        $(
            impl Int for $int {
                type Unsigned = $uint;
                type Signed = $sint;

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
                fn cast_unsigned(self) -> Self::Unsigned {
                    self as $uint
                }
                #[inline]
                fn cast_signed(self) -> Self::Signed {
                    self as $sint
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

impl_int!(
    i8 => u8, i8, i16 => u16, i16, i32 => u32, i32, i64 => u64, i64, isize => usize, isize
);
impl_int!(
    u8 => u8, i8, u16 => u16, i16, u32 => u32, i32, u64 => u64, i64, usize => usize, isize
);

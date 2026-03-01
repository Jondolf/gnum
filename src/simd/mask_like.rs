use core::ops::{BitAnd, BitOr, BitXor, Not};

/// Base trait for mask-like types used in conditional logic.
///
/// This trait is implemented by `bool` as well as SIMD mask types. It is designed to abstract
/// the behavior of booleans to enable the use of conditional logic in an [AoSoA] setting.
///
/// [AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA
pub trait MaskLike:
    Copy
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
{
    /// The true value for this type.
    const TRUE: Self;

    /// The false value for this type.
    const FALSE: Self;

    /// Returns a bit mask representing the boolean state of each mask bit.
    ///
    /// Set bits in the result correspond to `true` values in the mask,
    /// and unset bits correspond to `false` values.
    fn to_bitmask(self) -> u64;

    /// Returns `true` if all bits in the mask are set (`true`), and `false` otherwise.
    fn all(self) -> bool;

    /// Returns `true` if any bit in the mask is set (`true`), and `false` otherwise.
    fn any(self) -> bool;
}

impl MaskLike for bool {
    const TRUE: Self = true;
    const FALSE: Self = false;

    #[inline(always)]
    fn to_bitmask(self) -> u64 {
        self as u64
    }

    #[inline(always)]
    fn all(self) -> bool {
        self
    }

    #[inline(always)]
    fn any(self) -> bool {
        self
    }
}

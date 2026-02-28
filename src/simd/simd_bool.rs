use core::{
    ops::{BitAnd, BitOr, BitXor, Not},
    simd::{LaneCount, MaskElement, SupportedLaneCount},
};

/// Lane-wise generalization of `bool` for SIMD booleans.
///
/// This trait implemented by `bool` as well as SIMD mask types. It is designed to abstract
/// the behavior of booleans so it can work with multi-lane boolean values in an AoSoA setting.
pub trait SimdBool:
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

    /// A bit mask representing the boolean state of each lane of `self`.
    ///
    /// The `i-th` bit of the result is `1` iff. the `i-th` lane of `self` is `true`.
    fn to_bitmask(self) -> u64;

    /// Are all vector lanes true?
    fn all(self) -> bool;

    /// Is any vector lane true?
    fn any(self) -> bool;
}

impl SimdBool for bool {
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

impl<T: MaskElement, const N: usize> SimdBool for core::simd::Mask<T, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const TRUE: Self = Self::TRUE;
    const FALSE: Self = Self::FALSE;

    #[inline(always)]
    fn to_bitmask(self) -> u64 {
        self.to_bitmask()
    }

    #[inline(always)]
    fn all(self) -> bool {
        self.all()
    }

    #[inline(always)]
    fn any(self) -> bool {
        self.any()
    }
}

use crate::{
    num::{Int, Signed},
    simd::MaskLike,
};

/// Trait for converting between mask types and their integer representations.
pub trait MaskCast: MaskLike {
    /// The type of the integer representation of this mask.
    ///
    /// This is the type used for converting between masks and integers,
    /// and for casting between different mask types.
    type Int: Int + Signed;

    /// Converts a vector of integers to a mask, where 0 represents `false`
    /// and -1 represents `true`.
    ///
    /// # Panics
    ///
    /// Panics if any element is not 0 or -1.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn from_int(value: Self::Int) -> Self;

    /// Converts a vector of integers to a mask, where 0 represents `false`
    /// and -1 represents `true`, without checking that the input values are valid.
    ///
    /// # Safety
    ///
    /// Undefined behavior if any element is not 0 or -1.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    unsafe fn from_int_unchecked(value: Self::Int) -> Self;

    /// Converts the mask to a vector of integers, where 0 represents `false`
    /// and -1 represents `true`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn to_int(self) -> Self::Int;

    /// Casts this mask to another mask type with the same integer representation.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn cast<U: MaskCast<Int = Self::Int>>(self) -> U;
}

impl MaskCast for bool {
    type Int = i8;

    #[inline(always)]
    fn from_int(value: Self::Int) -> Self {
        debug_assert!(
            value == 0 || value == -1,
            "invalid integer value for bool mask"
        );
        value != 0
    }

    #[inline(always)]
    unsafe fn from_int_unchecked(value: Self::Int) -> Self {
        value != 0
    }

    #[inline(always)]
    fn to_int(self) -> Self::Int {
        if self { -1 } else { 0 }
    }

    #[inline(always)]
    fn cast<U: MaskCast<Int = Self::Int>>(self) -> U {
        U::from_int(self.to_int())
    }
}

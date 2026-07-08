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

    /// Tests the value of the specified element.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    #[must_use = "method returns a new bool and does not mutate the original value"]
    fn test(&self, index: usize) -> bool;

    /// Tests the value of the specified element without bounds checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `index` is greater than or equal to the number of elements in the vector.
    #[must_use = "method returns a new bool and does not mutate the original value"]
    unsafe fn test_unchecked(&self, index: usize) -> bool;

    /// Sets the value of the specified element.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    fn set(&mut self, index: usize, value: bool);

    /// Sets the value of the specified element without bounds checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `index` is greater than or equal to the number of elements in the vector.
    unsafe fn set_unchecked(&mut self, index: usize, value: bool);

    /// Returns a bit mask representing the boolean state of each mask bit.
    ///
    /// Set bits in the result correspond to `true` values in the mask,
    /// and unset bits correspond to `false` values.
    #[must_use = "method returns a new bitmask and does not mutate the original value"]
    fn to_bitmask(self) -> u64;

    /// Returns `true` if all bits in the mask are set (`true`), and `false` otherwise.
    #[must_use = "method returns a new bool and does not mutate the original value"]
    fn all(self) -> bool;

    /// Returns `true` if any bit in the mask is set (`true`), and `false` otherwise.
    #[must_use = "method returns a new bool and does not mutate the original value"]
    fn any(self) -> bool;
}

impl MaskLike for bool {
    const TRUE: Self = true;
    const FALSE: Self = false;

    #[inline(always)]
    fn test(&self, _index: usize) -> bool {
        debug_assert!(_index == 0, "index out of bounds for bool mask");
        *self
    }

    #[inline(always)]
    unsafe fn test_unchecked(&self, _index: usize) -> bool {
        *self
    }

    #[inline(always)]
    fn set(&mut self, _index: usize, value: bool) {
        debug_assert!(_index == 0, "index out of bounds for bool mask");
        *self = value;
    }

    #[inline(always)]
    unsafe fn set_unchecked(&mut self, _index: usize, value: bool) {
        *self = value;
    }

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

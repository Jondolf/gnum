use core::fmt;

use gnum::simd::{MaskCast, MaskLike, Select, SimdLike};

#[cfg(feature = "portable_simd")]
mod portable_simd;
#[cfg(all(feature = "wide", not(feature = "portable_simd")))]
mod wide;

mod sealed {
    pub trait Sealed<const N: usize> {}
}

/// Backend mapping for a SIMD element type and lane count.
#[doc(hidden)]
pub trait SimdElement<const N: usize>:
    sealed::Sealed<N> + Copy + fmt::Debug + 'static + SimdLike<Element = Self, Bool = bool>
{
    /// The backend vector type.
    ///
    /// The initialized representation must be bitwise compatible with `[Self; N]`.
    type Vector: Copy + fmt::Debug + SimdLike<Element = Self, Bool = Self::Mask, Array = [Self; N]>;

    /// The type of the integer representation of the mask type.
    type MaskInt: gnum::num::Int + gnum::num::Signed;

    /// The backend mask type.
    type Mask: fmt::Debug
        + MaskCast<Int = Self::MaskInt>
        + Select<Self::Vector>
        + Select<Self::Mask>;

    /// Returns a reference to the SIMD vector as an array.
    fn vector_as_array(vector: &Self::Vector) -> &[Self; N];

    /// Returns a mutable reference to the SIMD vector as an array.
    fn vector_as_mut_array(vector: &mut Self::Vector) -> &mut [Self; N];

    /// Creates a SIMD vector mask from an array.
    fn mask_from_array(array: [bool; N]) -> Self::Mask {
        let mut mask = <Self::Mask as MaskLike>::FALSE;
        for (index, value) in array.into_iter().enumerate() {
            mask.set(index, value);
        }
        mask
    }

    /// Converts a SIMD vector mask to an array.
    fn mask_to_array(mask: Self::Mask) -> [bool; N] {
        let bits = MaskLike::to_bitmask(mask);
        core::array::from_fn(|index| (bits & (1 << index)) != 0)
    }
}

/// Backend mapping for the integer type used by [`gnum::num::Real`].
#[doc(hidden)]
pub trait RealElement<const N: usize>: SimdElement<N> + gnum::num::Real {
    type I32Element: SimdElement<N>;
}

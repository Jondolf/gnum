use super::BackendVector;
use crate::{Simd, SimdElement};
use gnum::num::{Int, Signed};

/// Maps a SIMD element type to its corresponding unsigned and signed types.
#[doc(hidden)]
pub trait IntElement<const N: usize>:
    SimdElement<N> + Int<Unsigned = Self::UnsignedElement, Signed = Self::SignedElement>
{
    type UnsignedElement: IntElement<N, UnsignedElement = Self::UnsignedElement, SignedElement = Self::SignedElement>;
    type SignedElement: IntElement<N, UnsignedElement = Self::UnsignedElement, SignedElement = Self::SignedElement>
        + Signed<Unsigned = Self::UnsignedElement>;
}

type UnsignedElement<T, const N: usize> = <T as IntElement<N>>::UnsignedElement;
type SignedElement<T, const N: usize> = <T as IntElement<N>>::SignedElement;
type UnsignedVector<T, const N: usize> = BackendVector<UnsignedElement<T, N>, N>;
type SignedVector<T, const N: usize> = BackendVector<SignedElement<T, N>, N>;

impl<T, const N: usize> Int for Simd<T, N>
where
    T: IntElement<N>,
    BackendVector<T, N>: Int<Unsigned = UnsignedVector<T, N>, Signed = SignedVector<T, N>>,
    UnsignedVector<T, N>: Int<Unsigned = UnsignedVector<T, N>, Signed = SignedVector<T, N>>,
    SignedVector<T, N>: Int<Unsigned = UnsignedVector<T, N>, Signed = SignedVector<T, N>>
        + Signed<Unsigned = UnsignedVector<T, N>>,
{
    type Unsigned = Simd<UnsignedElement<T, N>, N>;
    type Signed = Simd<SignedElement<T, N>, N>;

    const BITS: u32 = <BackendVector<T, N> as Int>::BITS;

    #[inline]
    fn count_ones(self) -> Self::Unsigned {
        Simd::from_inner(self.0.count_ones())
    }

    #[inline]
    fn count_zeros(self) -> Self::Unsigned {
        Simd::from_inner(self.0.count_zeros())
    }

    #[inline]
    fn leading_zeros(self) -> Self::Unsigned {
        Simd::from_inner(self.0.leading_zeros())
    }

    #[inline]
    fn trailing_zeros(self) -> Self::Unsigned {
        Simd::from_inner(self.0.trailing_zeros())
    }

    #[inline]
    fn leading_ones(self) -> Self::Unsigned {
        Simd::from_inner(self.0.leading_ones())
    }

    #[inline]
    fn trailing_ones(self) -> Self::Unsigned {
        Simd::from_inner(self.0.trailing_ones())
    }

    #[inline]
    fn rotate_left(self, n: Self::Unsigned) -> Self {
        Self::from_inner(self.0.rotate_left(n.0))
    }

    #[inline]
    fn rotate_right(self, n: Self::Unsigned) -> Self {
        Self::from_inner(self.0.rotate_right(n.0))
    }

    #[inline]
    fn swap_bytes(self) -> Self {
        Self::from_inner(self.0.swap_bytes())
    }

    #[inline]
    fn reverse_bits(self) -> Self {
        Self::from_inner(self.0.reverse_bits())
    }

    #[inline]
    fn from_be(self) -> Self {
        Self::from_inner(self.0.from_be())
    }

    #[inline]
    fn from_le(self) -> Self {
        Self::from_inner(self.0.from_le())
    }

    #[inline]
    fn to_be(self) -> Self {
        Self::from_inner(self.0.to_be())
    }

    #[inline]
    fn to_le(self) -> Self {
        Self::from_inner(self.0.to_le())
    }

    #[inline]
    fn cast_unsigned(self) -> Self::Unsigned {
        Simd::from_inner(self.0.cast_unsigned())
    }

    #[inline]
    fn cast_signed(self) -> Self::Signed {
        Simd::from_inner(self.0.cast_signed())
    }

    #[inline]
    fn ilog(self, base: Self) -> Self::Unsigned {
        Simd::from_inner(self.0.ilog(base.0))
    }

    #[inline]
    fn ilog2(self) -> Self::Unsigned {
        Simd::from_inner(self.0.ilog2())
    }

    #[inline]
    fn ilog10(self) -> Self::Unsigned {
        Simd::from_inner(self.0.ilog10())
    }

    #[inline]
    fn pow(self, exp: Self::Unsigned) -> Self {
        Self::from_inner(self.0.pow(exp.0))
    }

    #[inline]
    fn isqrt(self) -> Self {
        Self::from_inner(self.0.isqrt())
    }
}

macro_rules! impl_int_element {
    ($($element:ty => $unsigned:ty, $signed:ty),* $(,)?) => {
        $(
            impl<const N: usize> IntElement<N> for $element
            where
                $element: SimdElement<N>,
                $unsigned: SimdElement<N>,
                $signed: SimdElement<N>,
            {
                type UnsignedElement = $unsigned;
                type SignedElement = $signed;
            }
        )*
    };
}

impl_int_element!(
    u8 => u8, i8,
    u16 => u16, i16,
    u32 => u32, i32,
    u64 => u64, i64,
    i8 => u8, i8,
    i16 => u16, i16,
    i32 => u32, i32,
    i64 => u64, i64,
);
#[cfg(feature = "portable_simd")]
impl_int_element!(usize => usize, isize, isize => usize, isize);

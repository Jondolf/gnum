use crate::{Simd, SimdElement};

// SAFETY: `Simd<T, N>` is transparent over a backend vector whose layout is the
//         same as `[T; N]`, as required by `SimdElement`'s array reference operations.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::KnownLayout for Simd<T, N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
    type PointerMetadata = ();
    type MaybeUninit = core::mem::MaybeUninit<Self>;
    const LAYOUT: zerocopy::DstLayout = zerocopy::DstLayout::for_type::<Self>();
    fn raw_from_ptr_len(bytes: core::ptr::NonNull<u8>, _: ()) -> core::ptr::NonNull<Self> {
        bytes.cast()
    }
    fn pointer_to_metadata(_: *mut Self) {}
}

// SAFETY: SIMD vectors contain no interior mutability.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::Immutable for Simd<T, N> {
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

// SAFETY: All bit patterns are valid whenever they are valid for the lane array.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::TryFromBytes for Simd<T, N>
where
    [T; N]: zerocopy::FromBytes,
{
    fn only_derive_is_allowed_to_implement_this_trait() {}
    fn is_bit_valid<A>(_: zerocopy::Maybe<'_, Self, A>) -> bool
    where
        A: zerocopy::invariant::Alignment,
    {
        true
    }
}

// SAFETY: An all-zero lane array is a valid SIMD vector.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::FromZeros for Simd<T, N>
where
    [T; N]: zerocopy::FromBytes,
{
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

// SAFETY: Every bit pattern valid for `[T; N]` is valid for the SIMD vector.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::FromBytes for Simd<T, N>
where
    [T; N]: zerocopy::FromBytes,
{
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

// SAFETY: The SIMD vector has the same initialized, padding-free representation
//         as its lane array whenever the lane array implements `IntoBytes`.
unsafe impl<T: SimdElement<N>, const N: usize> zerocopy::IntoBytes for Simd<T, N>
where
    [T; N]: zerocopy::IntoBytes,
{
    fn only_derive_is_allowed_to_implement_this_trait() {}
}

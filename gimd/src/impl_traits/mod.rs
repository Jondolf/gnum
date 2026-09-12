#[cfg(feature = "portable_simd")]
mod bytes;
mod cast;
mod cmp;
mod float;
mod int;
mod markers;
mod num;
mod ops;
mod signed;
mod simd;

use crate::SimdElement;

type BackendVector<T, const N: usize> = <T as SimdElement<N>>::Vector;
type BackendMask<T, const N: usize> = <T as SimdElement<N>>::Mask;

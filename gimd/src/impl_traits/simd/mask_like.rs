use super::super::BackendMask;
use crate::{Mask, SimdElement};
use gnum::simd::MaskLike;

impl<T: SimdElement<N>, const N: usize> MaskLike for Mask<T, N> {
    const TRUE: Self = Self(<BackendMask<T, N> as MaskLike>::TRUE);
    const FALSE: Self = Self(<BackendMask<T, N> as MaskLike>::FALSE);

    #[inline]
    fn test(&self, index: usize) -> bool {
        self.0.test(index)
    }

    #[inline]
    unsafe fn test_unchecked(&self, index: usize) -> bool {
        unsafe { self.0.test_unchecked(index) }
    }

    #[inline]
    fn set(&mut self, index: usize, value: bool) {
        self.0.set(index, value);
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        unsafe {
            self.0.set_unchecked(index, value);
        }
    }

    #[inline]
    fn to_bitmask(self) -> u64 {
        self.0.to_bitmask()
    }

    #[inline]
    fn all(self) -> bool {
        self.0.all()
    }

    #[inline]
    fn any(self) -> bool {
        self.0.any()
    }
}

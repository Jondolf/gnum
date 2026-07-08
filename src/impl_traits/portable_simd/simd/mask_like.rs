use crate::simd::MaskLike;
use core::simd::MaskElement;

impl<T: MaskElement, const N: usize> MaskLike for core::simd::Mask<T, N> {
    const TRUE: Self = Self::TRUE;
    const FALSE: Self = Self::FALSE;

    #[inline(always)]
    fn test(&self, index: usize) -> bool {
        self.test(index)
    }

    #[inline(always)]
    unsafe fn test_unchecked(&self, index: usize) -> bool {
        unsafe { self.test_unchecked(index) }
    }

    #[inline(always)]
    fn set(&mut self, index: usize, value: bool) {
        self.set(index, value)
    }

    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        unsafe { self.set_unchecked(index, value) }
    }

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

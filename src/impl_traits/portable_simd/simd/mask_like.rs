use crate::simd::MaskLike;
use core::simd::{LaneCount, MaskElement, SupportedLaneCount};

impl<T: MaskElement, const N: usize> MaskLike for core::simd::Mask<T, N>
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

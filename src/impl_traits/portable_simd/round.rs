//! The backing implementation of the rounding operations for [`core::simd`] types.
//!
//! `core::simd` maps `floor`, `ceil`, `round`, and `trunc` onto LLVM intrinsics,
//! which lower to a single instruction on some modern targets. On SSE2, these
//! instructions do not exist, so LLVM instead calls the scalar libm routine per lane.
//!
//! As an optimization, on a target without hardware rounding, everything here
//! is derived from an int round-trip that is bit-identical to the scalar result
//! but was measured to be 2-7x as fast.

#[allow(unused_imports, reason = "used on targets without hardware rounding")]
use crate::{cmp::NumEq, simd::Select};
#[allow(unused_imports, reason = "used on targets without hardware rounding")]
use core::simd::{
    Simd,
    cmp::SimdPartialOrd,
    num::{SimdFloat, SimdInt},
};
#[allow(unused_imports, reason = "used on targets with hardware rounding")]
use std::simd::StdFloat;

pub(crate) trait RoundOps: Copy {
    fn floor_internal(self) -> Self;
    fn ceil_internal(self) -> Self;
    fn round_internal(self) -> Self;
    fn round_ties_even_internal(self) -> Self;
    fn trunc_internal(self) -> Self;
}

/// We just use core::simd for targets with hardware rounding instructions.
#[cfg(any(
    target_feature = "sse4.1",
    target_feature = "simd128",
    all(target_arch = "aarch64", target_feature = "neon")
))]
macro_rules! impl_round {
    ($($real:ty => $int:ty, $uint:ty, $magic:expr, $sign:expr);* $(;)?) => {
        $(
            impl<const N: usize> RoundOps for Simd<$real, N> {
                #[inline(always)]
                fn floor_internal(self) -> Self {
                    StdFloat::floor(self)
                }

                #[inline(always)]
                fn ceil_internal(self) -> Self {
                    StdFloat::ceil(self)
                }

                #[inline(always)]
                fn round_internal(self) -> Self {
                    StdFloat::round(self)
                }

                #[inline(always)]
                fn round_ties_even_internal(self) -> Self {
                    StdFloat::round_ties_even(self)
                }

                #[inline(always)]
                fn trunc_internal(self) -> Self {
                    StdFloat::trunc(self)
                }
            }
        )*
    };
}

// For targets without hardware rounding, emulate everything using an int round-trip.
#[cfg(not(any(
    target_feature = "sse4.1",
    target_feature = "simd128",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
macro_rules! impl_round {
    ($($real:ty => $int:ty, $uint:ty, $magic:expr, $sign:expr);* $(;)?) => {
        $(
            impl<const N: usize> RoundOps for Simd<$real, N> {
                /// Rounds toward zero by casting to an int and back.
                #[inline(always)]
                fn trunc_internal(self) -> Self {
                    // See the bottom of this file for the magic numbers.
                    let magic = Simd::<$real, N>::splat($magic);

                    // Clamp first to make sure the values are in range, then perform an unchecked
                    // float-to-int cast. This was measured to be several times faster than a checked cast.
                    let clamped = self.simd_max(-magic).simd_min(magic);
                    let round_tripped =
                        unsafe { clamped.to_int_unchecked::<$int>() }.cast::<$real>();

                    // Keep the original value for any lane that was out of range.
                    // These are already integers, infinities, or NaN, which we want to preserve.
                    let in_range = self.abs().simd_lt(magic);
                    let truncated = Select::select(in_range, round_tripped, self);

                    // Preserve the original sign to handle signed zero correctly.
                    let sign = self.to_bits() & Simd::<$uint, N>::splat($sign);

                    // Cast back to the float type and restore the sign bit.
                    Simd::from_bits(truncated.to_bits() | sign)
                }

                /// Rounds toward negative infinity.
                #[inline(always)]
                fn floor_internal(self) -> Self {
                    // Truncate to get the integer part.
                    let t = self.trunc_internal();

                    // For negative non-integers, truncation overshoots, so step down by one there.
                    // Selecting, rather than adding a mask, leaves signed zero, infinities, and NaN intact.
                    Select::select(t.simd_gt(self), t - Simd::splat(1.0), t)
                }

                /// Rounds toward positive infinity.
                #[inline(always)]
                fn ceil_internal(self) -> Self {
                    // Truncate to get the integer part.
                    let t = self.trunc_internal();

                    // For positive non-integers, truncation undershoots, so step up by one there.
                    // Selecting, rather than adding a mask, leaves signed zero, infinities, and NaN intact.
                    Select::select(t.simd_lt(self), t + Simd::splat(1.0), t)
                }

                /// Rounds to the nearest integer, with ties away from zero.
                #[inline(always)]
                fn round_internal(self) -> Self {
                    // Truncate to get the integer part.
                    let t = self.trunc_internal();

                    // For each lane, pick `-1.0` or `1.0` depending on the sign of the original value.
                    let away = Select::select(
                        self.simd_lt(Simd::splat(0.0)),
                        Simd::splat(-1.0),
                        Simd::splat(1.0),
                    );

                    // For each lane, check whether the discarded fraction is at least a half.
                    // If so, step away from zero.
                    let has_half = (self - t).abs().simd_ge(Simd::splat(0.5));
                    let rounded = Select::select(has_half, t + away, t);

                    // `round` quiets a signaling NaN.
                    // We do this by adding `0.0` to the original value if it is NaN.
                    let is_nan = NumEq::num_ne(self, self);

                    Select::select(is_nan, self + Simd::splat(0.0), rounded)
                }

                /// Rounds to the nearest integer, with ties to even.
                #[inline(always)]
                fn round_ties_even_internal(self) -> Self {
                    // See the bottom of this file for the magic numbers.
                    let magic = Simd::<$real, N>::splat($magic);

                    // For each lane, pick `-magic` or `magic` depending on the sign of the original value.
                    let signed_magic =
                        Select::select(self.simd_lt(Simd::splat(0.0)), -magic, magic);

                    // This rounds to the nearest integer, with ties to even, via the FPU's own rounding.
                    let rounded = (self + signed_magic) - signed_magic;

                    // Keep the original value for any lane that was out of range.
                    // These are already integers, infinities, or NaN, which we want to preserve.
                    let in_range = self.abs().simd_lt(magic);
                    let result = Select::select(in_range, rounded, self);

                    // Preserve the original sign to handle signed zero correctly.
                    let sign = self.to_bits() & Simd::<$uint, N>::splat($sign);

                    // Cast back to the float type and restore the sign bit.
                    Simd::from_bits(result.to_bits() | sign)
                }
            }
        )*
    };
}

/// `2^(mantissa bits)`, or `2^23` for an `f32`.
///
/// This is the smallest magnitude whose ULP is `1.0`, so every `f32` at or above it
/// is already an integer and rounds to itself. This way, `(x + magic) - magic`
/// snaps smaller values to the nearest integer via the FPU's own rounding.
#[allow(dead_code, reason = "used on targets without hardware rounding")]
const F32_ROUND_MAGIC: f32 = (1u32 << (f32::MANTISSA_DIGITS - 1)) as f32;

/// `2^(mantissa bits)`, or `2^52` for an `f64`.
///
/// This is the `f64` counterpart of [`F32_ROUND_MAGIC`].
#[allow(dead_code, reason = "used on targets without hardware rounding")]
const F64_ROUND_MAGIC: f64 = (1u64 << (f64::MANTISSA_DIGITS - 1)) as f64;

/// Sign-bit masks, used to restore `-0.0` after a round-trip that loses it.
#[allow(dead_code, reason = "used on targets without hardware rounding")]
const F32_SIGN_MASK: u32 = 1 << (u32::BITS - 1);
#[allow(dead_code, reason = "used on targets without hardware rounding")]
const F64_SIGN_MASK: u64 = 1 << (u64::BITS - 1);

impl_round!(
    f32 => i32, u32, F32_ROUND_MAGIC, F32_SIGN_MASK;
    f64 => i64, u64, F64_ROUND_MAGIC, F64_SIGN_MASK;
);

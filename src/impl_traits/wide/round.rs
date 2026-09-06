//! The backing implementation of the rounding operations for [`wide`] types.

use crate::cmp::NumEq;
use crate::num::Num;
use crate::simd::Select;
use wide::*;

#[allow(unused_imports, reason = "used on targets without hardware rounding")]
use crate::cmp::NumOrd;
#[allow(unused_imports, reason = "used on targets without hardware rounding")]
use crate::num::{RealConstants, Signed};
#[allow(unused_imports, reason = "used on targets without hardware rounding")]
use crate::simd::MaskCast;

pub(crate) trait RoundOps: Copy {
    fn floor_internal(self) -> Self;
    fn ceil_internal(self) -> Self;
    fn round_internal(self) -> Self;
    fn round_ties_even_internal(self) -> Self;
    fn trunc_internal(self) -> Self;
}

/// Quiets a signaling NaN in `rounded`, wherever `original` was NaN.
#[inline(always)]
fn quiet_nan<T>(original: T, rounded: T) -> T
where
    T: Num + NumEq,
    T::Bool: Select<T>,
{
    original
        .num_ne(original)
        .select(original + T::ZERO, rounded)
}

/// We just use `wide` for targets with hardware rounding instructions.
#[cfg(any(
    target_feature = "sse4.1",
    target_feature = "simd128",
    all(target_arch = "aarch64", target_feature = "neon")
))]
macro_rules! impl_round {
    ($($simd:ident => $strategy:ident);* $(;)?) => {
        $(
            impl RoundOps for $simd {
                #[inline(always)]
                fn floor_internal(self) -> Self {
                    self.floor()
                }

                #[inline(always)]
                fn ceil_internal(self) -> Self {
                    self.ceil()
                }

                #[inline(always)]
                fn round_internal(self) -> Self {
                    // `wide` does not quiet a signaling NaN, but the scalar reference does.
                    quiet_nan(self, self.round())
                }

                #[inline(always)]
                fn round_ties_even_internal(self) -> Self {
                    self.round_ties_even()
                }

                #[inline(always)]
                fn trunc_internal(self) -> Self {
                    self.trunc()
                }
            }
        )*
    };
}

// For targets without hardware rounding, emulate `floor` and `ceil` using an int round-trip.
#[cfg(not(any(
    target_feature = "sse4.1",
    target_feature = "simd128",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
macro_rules! impl_round {
    ($($simd:ident => $strategy:ident);* $(;)?) => {
        $(
            impl RoundOps for $simd {
                /// Rounds toward zero.
                #[inline(always)]
                fn trunc_internal(self) -> Self {
                    self.trunc()
                }

                /// Rounds toward negative infinity.
                #[inline(always)]
                fn floor_internal(self) -> Self {
                    impl_round!(@floor $strategy, self)
                }

                /// Rounds toward positive infinity.
                #[inline(always)]
                fn ceil_internal(self) -> Self {
                    impl_round!(@ceil $strategy, self)
                }

                /// Rounds to the nearest integer, with ties away from zero.
                #[inline(always)]
                fn round_internal(self) -> Self {
                    impl_round!(@round $strategy, self)
                }

                /// Rounds to the nearest integer, with ties to even.
                #[inline(always)]
                fn round_ties_even_internal(self) -> Self {
                    // `wide` loses the sign of zero, so we must restore it.
                    self.round_ties_even() | (self & Self::splat(-0.0))
                }
            }
        )*
    };

    (@floor int_round_trip, $v:expr) => {{
        let v = $v;
        let t = v.fast_trunc_int().round_float();
        let floored = t + MaskCast::to_int(NumOrd::num_gt(t, v)).round_float();
        impl_round!(@in_range v, floored)
    }};

    (@ceil int_round_trip, $v:expr) => {{
        let v = $v;
        let t = v.fast_trunc_int().round_float();
        let ceiled = t - MaskCast::to_int(NumOrd::num_lt(t, v)).round_float();
        impl_round!(@in_range v, ceiled)
    }};

    (@in_range $v:expr, $rounded:expr) => {{
        let in_range = NumOrd::num_lt($v.abs(), Self::splat(F32_NO_FRACTION));
        let result = Select::select(in_range, $rounded, $v);
        result | ($v & Self::splat(-0.0))
    }};

    (@round int_round_trip, $v:expr) => {
        quiet_nan($v, $v.round())
    };

    (@floor from_trunc, $v:expr) => {{
        let v = $v;
        let t = v.trunc_internal();
        Select::select(NumOrd::num_gt(t, v), t - Self::ONE, t)
    }};

    (@ceil from_trunc, $v:expr) => {{
        let v = $v;
        let t = v.trunc_internal();
        Select::select(NumOrd::num_lt(t, v), t + Self::ONE, t)
    }};

    (@round from_trunc, $v:expr) => {{
        let v = $v;
        let t = v.trunc_internal();

        // For each lane, pick `-1.0` or `1.0` depending on the sign of the original value.
        let away = Select::select(NumOrd::num_lt(v, Self::ZERO), -Self::ONE, Self::ONE);

        // For each lane, step away from zero if the discarded fraction is at least a half.
        let has_half = NumOrd::num_ge(Signed::abs(v - t), Self::HALF);
        let rounded = Select::select(has_half, t + away, t);

        quiet_nan(v, rounded)
    }};
}

/// `2^(mantissa bits)`, or `2^23` for an `f32`.
///
/// This is the smallest magnitude whose ULP is `1.0`, so every `f32` at or above it
/// is already an integer and rounds to itself.
#[allow(dead_code, reason = "used on targets without hardware rounding")]
const F32_NO_FRACTION: f32 = (1u32 << (f32::MANTISSA_DIGITS - 1)) as f32;

impl_round!(
    f32x4 => int_round_trip;
    f32x8 => int_round_trip;
    f32x16 => int_round_trip;
    f64x2 => from_trunc;
    f64x4 => from_trunc;
    f64x8 => from_trunc;
);

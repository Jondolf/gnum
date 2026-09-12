//! Correctly rounded software fused multiply-add fallbacks.
//!
//! Adapted from the implementations in `libm` 0.2.16, licensed under MIT.
//!
//! <https://github.com/rust-lang/compiler-builtins/blob/dfd2203a4d6110820ad7bb65cafe1bf331a03a3d/libm/src/math/generic/fma_wide.rs#L5-L72>
//! <https://github.com/rust-lang/compiler-builtins/blob/dfd2203a4d6110820ad7bb65cafe1bf331a03a3d/libm/src/math/generic/fma.rs#L8-L278>
//! <https://github.com/rust-lang/compiler-builtins/blob/dfd2203a4d6110820ad7bb65cafe1bf331a03a3d/libm/src/math/generic/scalbn.rs#L20-L116>

#![allow(
    dead_code,
    reason = "software fallbacks are unused when every enabled backend has hardware FMA"
)]

/// FMA implementation when a hardware-backed larger float type is available. `f64` has enough
/// precision to represent `f32` in its entirety, except for double rounding.
#[inline]
pub(crate) fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    let one = 1_u64;

    let xy = x as f64 * y as f64;
    let result = xy + z as f64;
    let mut ui = result.to_bits();
    let re = (ui >> 52) & 0x7ff;
    let zb = z as f64;

    let prec_diff = 52 - 23;
    let excess_prec = ui & ((one << prec_diff) - one);
    let halfway = one << (prec_diff - 1);

    // Common case: the larger precision is fine if...
    // This is not a halfway case
    if excess_prec != halfway
        // Or the result is NaN or infinity
        || re == 0x7ff
        // Or the result is exact
        || (result - xy == zb && result - zb == xy)
    {
        return result as f32;
    }

    let neg = ui >> 63 != 0;
    let err = if neg == (zb > xy) {
        xy - result + zb
    } else {
        zb - result + xy
    };
    if neg == (err < 0.0) {
        ui += one;
    } else {
        ui -= one;
    }

    f64::from_bits(ui) as f32
}

/// Fused multiply-add that works when there is not a larger float size available.
/// Computes `(x * y) + z`.
#[inline]
pub(crate) fn fma(x: f64, y: f64, z: f64) -> f64 {
    let one = 1_u64;
    let zero = 0_u64;

    // Normalize such that the top of the mantissa is zero and we have a guard bit.
    let nx = Norm::from_float(x);
    let ny = Norm::from_float(y);
    let nz = Norm::from_float(z);

    if nx.is_zero_nan_inf() || ny.is_zero_nan_inf() {
        // Value will overflow, defer to non-fused operations.
        return x * y + z;
    }

    if nz.is_zero_nan_inf() {
        if nz.is_zero() {
            // Empty add component means we only need to multiply.
            return x * y;
        }
        // `z` is NaN or infinity, which sets the result.
        return z;
    }

    // multiply: r = x * y
    let zhi: u64;
    let zlo: u64;
    let product = nx.m as u128 * ny.m as u128;
    let mut rlo = product as u64;
    let mut rhi = (product >> 64) as u64;

    // Exponent result of multiplication
    let mut e = nx.e + ny.e;
    // Needed shift to align `z` to the multiplication result
    let mut d = nz.e - e;
    let sbits = 64;

    // Scale `z`. Shift `z <<= kz`, `r >>= kr`, so `kz+kr == d`, set `e = e+kr` (== ez-kz)
    if d > 0 {
        // The magnitude of `z` is larger than `x * y`
        if d < sbits {
            // Maximum shift of one `f64::BITS` means shifted `z` will fit into 128 bits. Shift it
            // into `(zhi, zlo)`. No exponent adjustment necessary.
            zlo = nz.m << d;
            zhi = nz.m >> (sbits - d);
        } else {
            // Shift larger than `sbits`, `z` only needs the top half `zhi`. Place it there (acts
            // as a shift by `sbits`).
            zlo = zero;
            zhi = nz.m;
            d -= sbits;

            // `z`'s exponent is large enough that it now needs to be taken into account.
            e = nz.e - sbits;

            if d == 0 {
                // Exactly `sbits`, nothing to do
            } else if d < sbits {
                // Remaining shift fits within `sbits`. Leave `z` in place, shift `x * y`
                rlo = (rhi << (sbits - d)) | (rlo >> d);
                // Set the sticky bit
                rlo |= ((rlo << (sbits - d)) != zero) as u64;
                rhi >>= d;
            } else {
                // `z`'s magnitude is enough that `x * y` is irrelevant. It was nonzero, so set
                // the sticky bit.
                rlo = one;
                rhi = zero;
            }
        }
    } else {
        // `z`'s magnitude once shifted fits entirely within `zlo`
        zhi = zero;
        d = -d;
        if d == 0 {
            // No shift needed
            zlo = nz.m;
        } else if d < sbits {
            // Shift s.t. `nz.m` fits into `zlo`
            let sticky = ((nz.m << (sbits - d)) != zero) as u64;
            zlo = (nz.m >> d) | sticky;
        } else {
            // Would be entirely shifted out, only set the sticky bit
            zlo = one;
        }
    }

    /* addition */

    let mut neg = nx.neg ^ ny.neg;
    let samesign = !neg ^ nz.neg;
    let mut rhi_nonzero = true;

    if samesign {
        // r += z
        let (res, carry) = rlo.overflowing_add(zlo);
        rlo = res;
        rhi = rhi.wrapping_add(zhi).wrapping_add(carry as u64);
    } else {
        // r -= z
        let (res, borrow) = rlo.overflowing_sub(zlo);
        rlo = res;
        rhi = rhi.wrapping_sub(zhi.wrapping_add(borrow as u64));
        if rhi >> 63 != zero {
            rlo = (rlo as i64).wrapping_neg() as u64;
            rhi = (rhi as i64)
                .wrapping_neg()
                .wrapping_sub((rlo != zero) as i64) as u64;
            neg = !neg;
        }
        rhi_nonzero = rhi != zero;
    }

    /* Construct result */

    // Shift result into `rhi`, left-aligned. Last bit is sticky
    if rhi_nonzero {
        // `d` > 0, need to shift both `rhi` and `rlo` into result
        e += sbits;
        d = rhi.leading_zeros() as i32 - 1;
        rhi = (rhi << d) | (rlo >> (sbits - d));
        // Update sticky
        rhi |= ((rlo << d) != zero) as u64;
    } else if rlo != zero {
        // `rhi` is zero, `rlo` is the entire result and needs to be shifted
        d = rlo.leading_zeros() as i32 - 1;
        if d < 0 {
            // Shift and set sticky
            rhi = (rlo >> 1) | (rlo & one);
        } else {
            rhi = rlo << d;
        }
    } else {
        // exact +/- 0.0
        return x * y + z;
    }

    e -= d;

    // Use int->float conversion to populate the significand.
    // `i` is in `[1 << 62, (1 << 63) - 1]`.
    let mut i = rhi as i64;

    if neg {
        i = -i;
    }

    // `|r|` is in `[0x1p62,0x1p63]` for `f64`
    let mut r = i as f64;

    /* Account for subnormal and rounding */

    // Unbiased exponent for the maximum value of `r`
    let max_pow = 64 - 1 + 1023;

    if e < -(max_pow - 2) {
        // Result is subnormal before rounding
        if e == -(max_pow - 1) {
            let mut c = f64::from_bits((max_pow as u64) << 52);
            if neg {
                c = -c;
            }

            if r == c {
                // Min normal after rounding
                return f64::MIN_POSITIVE.copysign(r);
            }

            if rhi << (52 + 1) != zero {
                // Account for truncated bits. One bit will be lost in the `scalbn` call, add
                // another top bit to avoid double rounding if inexact.
                let iu = (rhi >> 1) | (rhi & one) | (one << 62);
                i = iu as i64;

                if neg {
                    i = -i;
                }

                r = i as f64;

                // Remove the top bit
                r = 2.0 * r - c;
            }
        } else {
            // Only round once when scaled
            d = 11 - 1;
            let sticky = ((rhi << (64 - d)) != zero) as u64;
            i = (((rhi >> d) | sticky) << d) as i64;

            if neg {
                i = -i;
            }

            r = i as f64;
        }
    }

    // Use our exponent to scale the final value.
    scalbn(r, e)
}

/// Representation of `f64` that has handled subnormals.
#[derive(Clone, Copy, Debug)]
struct Norm {
    /// Normalized significand with one guard bit, unsigned.
    m: u64,
    /// Exponent of the mantissa such that `m * 2^e = x`. Accounts for the shift in the mantissa
    /// and the guard bit; that is, 1.0 will normalize as `m = 1 << 53` and `e = -53`.
    e: i32,
    neg: bool,
}

impl Norm {
    /// Unbias the exponent and account for the mantissa's precision, including the guard bit.
    const EXP_UNBIAS: u32 = 1023 + 52 + 1;

    /// Values greater than this had a saturated exponent (infinity or NaN), or zero was adjusted
    /// to have an exponent greater than this value.
    const ZERO_INF_NAN: u32 = 2047 - Self::EXP_UNBIAS;

    fn from_float(x: f64) -> Self {
        let mut ix = x.to_bits();
        let mut e = ((ix >> 52) & 0x7ff) as i32;
        let neg = x.is_sign_negative();
        if e == 0 {
            // Normalize subnormals by multiplication
            let scale_i = 64 - 1;
            let scale_f = f64::from_bits(((scale_i + 1023) as u64) << 52);
            let scaled = x * scale_f;
            ix = scaled.to_bits();
            e = ((ix >> 52) & 0x7ff) as i32;
            e = if e == 0 {
                // If the exponent is zero, set it above `ZERO_INF_NAN` to represent zero.
                1 << 11
            } else {
                // Account for the normalization scale.
                e - scale_i
            };
        }

        e -= Self::EXP_UNBIAS as i32;

        // Absolute value, set the implicit bit, and shift to create a guard bit.
        ix &= (1_u64 << 52) - 1;
        ix |= 1_u64 << 52;
        ix <<= 1;

        Self { m: ix, e, neg }
    }

    /// True if the value was zero, infinity, or NaN.
    fn is_zero_nan_inf(self) -> bool {
        self.e >= Self::ZERO_INF_NAN as i32
    }

    fn is_zero(self) -> bool {
        // Only the zero sentinel has an exponent greater than `ZERO_INF_NAN`.
        self.e > Self::ZERO_INF_NAN as i32
    }
}

/// Multiplies by `2^n` without prematurely rounding a subnormal.
#[inline]
fn scalbn(mut x: f64, mut n: i32) -> f64 {
    const MAX_EXP: i32 = 1023;
    const MIN_EXP: i32 = -1022;
    const MIN_NORMAL: f64 = f64::from_bits(1_u64 << 52);
    const NORMALIZE_SUBNORMAL: f64 = f64::from_bits((1023_u64 + 53) << 52);
    const MAX_SCALE: f64 = f64::from_bits((1023_u64 << 1) << 52);

    if n > MAX_EXP {
        x *= MAX_SCALE;
        n -= MAX_EXP;
        if n > MAX_EXP {
            x *= MAX_SCALE;
            n = (n - MAX_EXP).min(MAX_EXP);
        }
    } else if n < MIN_EXP {
        let scale = MIN_NORMAL * NORMALIZE_SUBNORMAL;
        let adjustment = -MIN_EXP - 53;
        x *= scale;
        n += adjustment;
        if n < MIN_EXP {
            x *= scale;
            n = (n + adjustment).max(MIN_EXP);
        }
    }

    x * f64::from_bits(((1023 + n) as u64) << 52)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn same_or_both_nan_f32(actual: f32, expected: f32) -> bool {
        actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan())
    }

    fn same_or_both_nan_f64(actual: f64, expected: f64) -> bool {
        actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan())
    }

    #[test]
    fn matches_native_fma() {
        let casesf32 = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            f32::MIN_POSITIVE,
            f32::from_bits(1),
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
        ];
        for &x in &casesf32 {
            for &y in &casesf32 {
                for &z in &casesf32 {
                    assert!(same_or_both_nan_f32(fmaf(x, y, z), x.mul_add(y, z)));
                }
            }
        }

        let casesf64 = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
        ];
        for &x in &casesf64 {
            for &y in &casesf64 {
                for &z in &casesf64 {
                    assert!(same_or_both_nan_f64(fma(x, y, z), x.mul_add(y, z)));
                }
            }
        }

        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        for _ in 0..500_000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let x = f32::from_bits(state as u32);
            state = state.rotate_left(29).wrapping_add(0xa076_1d64_78bd_642f);
            let y = f32::from_bits(state as u32);
            state = state.rotate_left(31).wrapping_add(0xe703_7ed1_a0b4_28db);
            let z = f32::from_bits(state as u32);
            assert!(same_or_both_nan_f32(fmaf(x, y, z), x.mul_add(y, z)));

            let x = f64::from_bits(state);
            state = state.rotate_left(27).wrapping_add(0x8ebc_6af0_9c88_c6e3);
            let y = f64::from_bits(state);
            state = state.rotate_left(33).wrapping_add(0x5899_65cc_7537_4cc3);
            let z = f64::from_bits(state);
            assert!(
                same_or_both_nan_f64(fma(x, y, z), x.mul_add(y, z)),
                "x={x:?}, y={y:?}, z={z:?}"
            );
        }
    }
}

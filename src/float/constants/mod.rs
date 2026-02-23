#[cfg(feature = "portable_simd")]
mod portable_simd;
mod scalar;

pub trait FloatConst {
    const E: Self;
    const FRAC_1_PI: Self;
    const FRAC_1_SQRT_2: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const LN_2: Self;
    const LN_10: Self;
    const LOG2_10: Self;
    const LOG2_E: Self;
    const LOG10_2: Self;
    const LOG10_E: Self;
    const PI: Self;
    const SQRT_2: Self;
    const TAU: Self;
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self;
    #[cfg(feature = "more_float_constants")]
    const PHI: Self;
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self;
}

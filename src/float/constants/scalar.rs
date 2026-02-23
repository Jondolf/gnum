use super::FloatConst;

impl FloatConst for f32 {
    const E: Self = core::f32::consts::E;
    const FRAC_1_PI: Self = core::f32::consts::FRAC_1_PI;
    const FRAC_1_SQRT_2: Self = core::f32::consts::FRAC_1_SQRT_2;
    const FRAC_2_PI: Self = core::f32::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: Self = core::f32::consts::FRAC_2_SQRT_PI;
    const FRAC_PI_2: Self = core::f32::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = core::f32::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = core::f32::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = core::f32::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = core::f32::consts::FRAC_PI_8;
    const LN_2: Self = core::f32::consts::LN_2;
    const LN_10: Self = core::f32::consts::LN_10;
    const LOG2_10: Self = core::f32::consts::LOG2_10;
    const LOG2_E: Self = core::f32::consts::LOG2_E;
    const LOG10_2: Self = core::f32::consts::LOG10_2;
    const LOG10_E: Self = core::f32::consts::LOG10_E;
    const PI: Self = core::f32::consts::PI;
    const SQRT_2: Self = core::f32::consts::SQRT_2;
    const TAU: Self = core::f32::consts::TAU;
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self = core::f32::consts::EGAMMA;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self = core::f32::consts::FRAC_1_SQRT_3;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self = core::f32::consts::FRAC_1_SQRT_2PI;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self = core::f32::consts::FRAC_1_SQRT_PI;
    #[cfg(feature = "more_float_constants")]
    const PHI: Self = core::f32::consts::PHI;
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self = core::f32::consts::SQRT_3;
}

impl FloatConst for f64 {
    const E: Self = core::f64::consts::E;
    const FRAC_1_PI: Self = core::f64::consts::FRAC_1_PI;
    const FRAC_1_SQRT_2: Self = core::f64::consts::FRAC_1_SQRT_2;
    const FRAC_2_PI: Self = core::f64::consts::FRAC_2_PI;
    const FRAC_2_SQRT_PI: Self = core::f64::consts::FRAC_2_SQRT_PI;
    const FRAC_PI_2: Self = core::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = core::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = core::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = core::f64::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = core::f64::consts::FRAC_PI_8;
    const LN_2: Self = core::f64::consts::LN_2;
    const LN_10: Self = core::f64::consts::LN_10;
    const LOG2_10: Self = core::f64::consts::LOG2_10;
    const LOG2_E: Self = core::f64::consts::LOG2_E;
    const LOG10_2: Self = core::f64::consts::LOG10_2;
    const LOG10_E: Self = core::f64::consts::LOG10_E;
    const PI: Self = core::f64::consts::PI;
    const SQRT_2: Self = core::f64::consts::SQRT_2;
    const TAU: Self = core::f64::consts::TAU;
    #[cfg(feature = "more_float_constants")]
    const EGAMMA: Self = core::f64::consts::EGAMMA;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_3: Self = core::f64::consts::FRAC_1_SQRT_3;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_2PI: Self = core::f64::consts::FRAC_1_SQRT_2PI;
    #[cfg(feature = "more_float_constants")]
    const FRAC_1_SQRT_PI: Self = core::f64::consts::FRAC_1_SQRT_PI;
    #[cfg(feature = "more_float_constants")]
    const PHI: Self = core::f64::consts::PHI;
    #[cfg(feature = "more_float_constants")]
    const SQRT_3: Self = core::f64::consts::SQRT_3;
}

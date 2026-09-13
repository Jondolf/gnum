//! The shared backing interface for scalar and SIMD rounding operations.

#[allow(unused, reason = "only used with some feature configurations")]
pub(crate) trait RoundOps: Copy {
    fn floor_internal(self) -> Self;
    fn ceil_internal(self) -> Self;
    fn round_internal(self) -> Self;
    fn round_ties_even_internal(self) -> Self;
    fn trunc_internal(self) -> Self;
}

#[cfg(not(feature = "std"))]
macro_rules! impl_round {
    ($float:ty, $uint:ty, $mantissa_bits:expr, $exponent_bias:expr) => {
        impl RoundOps for $float {
            #[inline]
            fn trunc_internal(self) -> Self {
                let bits = self.to_bits();
                let exponent = ((bits >> $mantissa_bits) & ((1 << (<$uint>::BITS - $mantissa_bits - 1)) - 1)) as i32
                    - $exponent_bias;

                if exponent < 0 {
                    return <$float>::from_bits(bits & (1 << (<$uint>::BITS - 1)));
                }
                if exponent >= $mantissa_bits as i32 {
                    return self;
                }

                let fractional = ((1 as $uint) << ($mantissa_bits - exponent as u32)) - 1;
                <$float>::from_bits(bits & !fractional)
            }

            #[inline]
            fn floor_internal(self) -> Self {
                let truncated = self.trunc_internal();
                if truncated > self { truncated - 1.0 } else { truncated }
            }

            #[inline]
            fn ceil_internal(self) -> Self {
                let truncated = self.trunc_internal();
                if truncated < self { truncated + 1.0 } else { truncated }
            }

            #[inline]
            fn round_internal(self) -> Self {
                let truncated = self.trunc_internal();
                let fraction = self - truncated;
                let rounded = if fraction >= 0.5 {
                    truncated + 1.0
                } else if fraction <= -0.5 {
                    truncated - 1.0
                } else {
                    truncated
                };
                if self.is_nan() { self + 0.0 } else { rounded }
            }

            #[inline]
            fn round_ties_even_internal(self) -> Self {
                const MAGIC: $float = (1 as $uint << $mantissa_bits) as $float;
                if self.abs() >= MAGIC {
                    return self;
                }
                let signed_magic = if self.is_sign_negative() { -MAGIC } else { MAGIC };
                let rounded = (self + signed_magic) - signed_magic;
                rounded.copysign(self)
            }
        }
    };
}

#[cfg(not(feature = "std"))]
impl_round!(f32, u32, 23, 127);
#[cfg(not(feature = "std"))]
impl_round!(f64, u64, 52, 1023);

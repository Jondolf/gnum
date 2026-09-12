use super::BackendVector;
use crate::backend::RealElement;
use crate::{Mask, Simd, SimdElement};
use gnum::{
    cmp::NumOrd,
    num::{Float, Int, Real, RealConstants, Signed},
};

impl<T, const N: usize> RealConstants for Simd<T, N>
where
    T: SimdElement<N>,
    BackendVector<T, N>: RealConstants,
{
    const HALF: Self = Self(<BackendVector<T, N> as RealConstants>::HALF);
    const E: Self = Self(<BackendVector<T, N> as RealConstants>::E);
    const EULER_GAMMA: Self = Self(<BackendVector<T, N> as RealConstants>::EULER_GAMMA);
    const FRAC_1_PI: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_1_PI);
    const FRAC_1_SQRT_2: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_1_SQRT_2);
    const FRAC_2_PI: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_2_PI);
    const FRAC_2_SQRT_PI: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_2_SQRT_PI);
    const FRAC_PI_2: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_PI_2);
    const FRAC_PI_3: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_PI_3);
    const FRAC_PI_4: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_PI_4);
    const FRAC_PI_6: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_PI_6);
    const FRAC_PI_8: Self = Self(<BackendVector<T, N> as RealConstants>::FRAC_PI_8);
    const GOLDEN_RATIO: Self = Self(<BackendVector<T, N> as RealConstants>::GOLDEN_RATIO);
    const LN_2: Self = Self(<BackendVector<T, N> as RealConstants>::LN_2);
    const LN_10: Self = Self(<BackendVector<T, N> as RealConstants>::LN_10);
    const LOG2_10: Self = Self(<BackendVector<T, N> as RealConstants>::LOG2_10);
    const LOG2_E: Self = Self(<BackendVector<T, N> as RealConstants>::LOG2_E);
    const LOG10_2: Self = Self(<BackendVector<T, N> as RealConstants>::LOG10_2);
    const LOG10_E: Self = Self(<BackendVector<T, N> as RealConstants>::LOG10_E);
    const PI: Self = Self(<BackendVector<T, N> as RealConstants>::PI);
    const SQRT_2: Self = Self(<BackendVector<T, N> as RealConstants>::SQRT_2);
    const TAU: Self = Self(<BackendVector<T, N> as RealConstants>::TAU);
}

macro_rules! real_unary_methods {
    ($($method:ident),* $(,)?) => {
        $(
            #[inline]
            fn $method(self) -> Self {
                Self::from_inner(self.0.$method())
            }
        )*
    };
}

macro_rules! real_binary_methods {
    ($($method:ident),* $(,)?) => {
        $(
            #[inline]
            fn $method(self, other: Self) -> Self {
                Self::from_inner(self.0.$method(other.0))
            }
        )*
    };
}

type RealI32<T, const N: usize> = <T as RealElement<N>>::I32Element;

impl<T, const N: usize> Real for Simd<T, N>
where
    T: RealElement<N>,
    Self: Signed + NumOrd<Bool = Mask<T, N>>,
    Simd<RealI32<T, N>, N>: Int,
    BackendVector<T, N>: Real<I32 = BackendVector<RealI32<T, N>, N>>,
{
    type I32 = Simd<RealI32<T, N>, N>;

    #[inline]
    fn from_f32(n: f32) -> Self {
        Self::from_inner(<BackendVector<T, N> as Real>::from_f32(n))
    }

    #[inline]
    fn from_f64(n: f64) -> Self {
        Self::from_inner(<BackendVector<T, N> as Real>::from_f64(n))
    }

    real_unary_methods! {
        floor, ceil, round, round_ties_even, trunc, fract, sqrt,
        exp, exp_stable, exp2, exp2_stable,
    }
    real_binary_methods! { log, log_stable }
    real_unary_methods! {
        ln, ln_stable, log2, log2_stable, log10, log10_stable, cbrt, cbrt_stable,
    }
    real_binary_methods! { hypot, hypot_stable }
    real_unary_methods! {
        sin, sin_stable, cos, cos_stable, tan, tan_stable,
        asin, asin_stable, acos, acos_stable, atan, atan_stable,
    }
    real_binary_methods! { atan2, atan2_stable }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self::from_inner(sin), Self::from_inner(cos))
    }

    #[inline]
    fn sin_cos_stable(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos_stable();
        (Self::from_inner(sin), Self::from_inner(cos))
    }

    real_unary_methods! {
        sinh, sinh_stable, cosh, cosh_stable, tanh, tanh_stable,
        asinh, asinh_stable, acosh, acosh_stable, atanh, atanh_stable,
        recip, to_degrees, to_radians,
    }
    real_binary_methods! { midpoint, copysign }
}

macro_rules! float_mask_methods {
    ($($method:ident),* $(,)?) => {
        $(
            #[inline]
            fn $method(self) -> Self::Bool {
                Mask::from_inner(self.0.$method())
            }
        )*
    };
}

type Bits<T> = <T as Float>::Bits;
type SignedBits<T> = <Bits<T> as Int>::Signed;

impl<T, const N: usize> Float for Simd<T, N>
where
    T: SimdElement<N> + Float,
    Bits<T>: SimdElement<N>,
    SignedBits<T>: SimdElement<N>,
    Self: Real<Bool = Mask<T, N>>,
    Simd<Bits<T>, N>: Int<Unsigned = Simd<Bits<T>, N>, Signed = Simd<SignedBits<T>, N>>,
    BackendVector<Bits<T>, N>:
        Int<Unsigned = BackendVector<Bits<T>, N>, Signed = BackendVector<SignedBits<T>, N>>,
    BackendVector<T, N>: Float<Bits = BackendVector<Bits<T>, N>>,
{
    type Bits = Simd<Bits<T>, N>;

    const RADIX: u32 = <BackendVector<T, N> as Float>::RADIX;
    const MANTISSA_DIGITS: u32 = <BackendVector<T, N> as Float>::MANTISSA_DIGITS;
    const DIGITS: u32 = <BackendVector<T, N> as Float>::DIGITS;
    const EPSILON: Self = Self(<BackendVector<T, N> as Float>::EPSILON);
    const MIN_POSITIVE: Self = Self(<BackendVector<T, N> as Float>::MIN_POSITIVE);
    const MIN_EXP: i32 = <BackendVector<T, N> as Float>::MIN_EXP;
    const MAX_EXP: i32 = <BackendVector<T, N> as Float>::MAX_EXP;
    const MIN_10_EXP: i32 = <BackendVector<T, N> as Float>::MIN_10_EXP;
    const MAX_10_EXP: i32 = <BackendVector<T, N> as Float>::MAX_10_EXP;
    const NAN: Self = Self(<BackendVector<T, N> as Float>::NAN);
    const INFINITY: Self = Self(<BackendVector<T, N> as Float>::INFINITY);
    const NEG_INFINITY: Self = Self(<BackendVector<T, N> as Float>::NEG_INFINITY);

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::from_inner(self.0.mul_add(a.0, b.0))
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        Self::from_inner(self.0.powf(n.0))
    }

    #[inline]
    fn powf_stable(self, n: Self) -> Self {
        Self::from_inner(self.0.powf_stable(n.0))
    }

    float_mask_methods! {
        is_nan, is_infinite, is_finite, is_subnormal, is_normal,
        is_sign_positive, is_sign_negative,
    }

    #[inline]
    fn next_up(self) -> Self {
        Self::from_inner(self.0.next_up())
    }

    #[inline]
    fn next_down(self) -> Self {
        Self::from_inner(self.0.next_down())
    }

    #[inline]
    fn from_bits(bits: Self::Bits) -> Self {
        Self::from_inner(<BackendVector<T, N> as Float>::from_bits(bits.0))
    }

    #[inline]
    fn to_bits(self) -> Self::Bits {
        Simd::from_inner(self.0.to_bits())
    }

    #[inline]
    fn from_int(int: <Self::Bits as Int>::Signed) -> Self {
        Self::from_inner(<BackendVector<T, N> as Float>::from_int(int.0))
    }

    #[inline]
    fn to_int(self) -> <Self::Bits as Int>::Signed {
        Simd::from_inner(self.0.to_int())
    }
}

use crate::{
    simd::SimdValue,
    traits::{Float, Int, Real},
};
use core::ops::*;

/// Base trait for numeric types.
///
/// This trait is implemented by all numeric types, including both scalar types and SIMD vector types.
/// It encapsulates the common properties of numeric types, such as having a minimum and maximum value, and
/// supporting basic arithmetic operations.
///
/// See [`Int`] and [`Float`] for more specific traits for integer and floating-point types, respectively.
///
/// [`Int`]: crate::traits::Int
/// [`Float`]: crate::traits::Float
pub trait Num: Copy + Zero + One + NumOps + SimdValue {
    /// The smallest finite value that can be represented by this type.
    const MIN: Self;
    /// The largest finite value that can be represented by this type.
    const MAX: Self;
}

macro_rules! impl_num_scalar {
    ($($t:ty),*) => {
        $(
            impl Num for $t {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;
            }
        )*
    };
}

impl_num_scalar!(u8, u16, u32, u64, usize);
impl_num_scalar!(i8, i16, i32, i64, isize);
impl_num_scalar!(f32, f64);

/// A trait for types that have an additive identity element `0`.
pub trait Zero: Sized {
    /// The additive identity element `0` for this type.
    const ZERO: Self;
}

/// A trait for types that have a multiplicative identity element `1`.
pub trait One: Sized {
    /// The multiplicative identity element `1` for this type.
    const ONE: Self;
}

/// A trait for types that have a multiplicative identity element `-1`.
pub trait NegOne: Sized {
    /// The multiplicative identity element `-1` for this type.
    const NEG_ONE: Self;
}

macro_rules! impl_zero_one_scalar {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0 as $t;
            }

            impl One for $t {
                const ONE: Self = 1 as $t;
            }
        )*
    };
}

impl_zero_one_scalar!(u8, u16, u32, u64, usize);
impl_zero_one_scalar!(i8, i16, i32, i64, isize);
impl_zero_one_scalar!(f32, f64);

macro_rules! impl_neg_one_scalar {
    ($($t:ty),*) => {
        $(
            impl NegOne for $t {
                const NEG_ONE: Self = -1 as $t;
            }
        )*
    };
}

impl_neg_one_scalar!(i8, i16, i32, i64, isize);
impl_neg_one_scalar!(f32, f64);

/// A trait for types that support basic arithmetic operations:
///
/// - Addition ([`+`](Add))
/// - Subtraction ([`-`](Sub))
/// - Multiplication ([`*`](Mul))
/// - Division ([`/`](Div))
/// - Remainder ([`%`](Rem))
pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
    + AddAssign<Rhs>
    + SubAssign<Rhs>
    + MulAssign<Rhs>
    + DivAssign<Rhs>
    + RemAssign<Rhs>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
        + AddAssign<Rhs>
        + SubAssign<Rhs>
        + MulAssign<Rhs>
        + DivAssign<Rhs>
        + RemAssign<Rhs>
{
}

/// A trait for types that support basic arithmetic operations with both owned and reference operands.
///
/// This trait is automatically implemented for any type that implements `NumOps` for both owned and reference operands.
pub trait NumRefOps<Rhs = Self, Output = Self>:
    NumOps<Rhs, Output> + for<'a> NumOps<&'a Rhs, Output>
{
}

/// A trait for types that support basic arithmetic operations with both owned and reference operands,
/// where the reference operand is the same type as the implementing type.
///
/// This trait is automatically implemented for any type that implements `NumOps` for both owned and reference operands of the same type.
pub trait RefNumOps<Rhs = Self, Output = Self>: for<'a> NumOps<&'a Rhs, Output> {}

impl<T> NumRefOps for T where T: NumOps + for<'a> NumOps<&'a T> {}
impl<T> RefNumOps for T where T: for<'a> NumOps<&'a T> {}

/// A marker trait for scalar [`Real`] types.
///
/// This trait is implemented by all scalar types that implement [`Real`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarReal: Real<Bool = bool> + PartialEq + PartialOrd {}

/// A marker trait for scalar [`Float`] types.
///
/// This trait is implemented by all scalar types that implement [`Float`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarFloat: Float<Bool = bool> + PartialEq + PartialOrd {}

/// A marker trait for scalar [`Int`] types.
///
/// This trait is implemented by all scalar types that implement [`Int`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarInt: Int<Bool = bool> + PartialEq + PartialOrd {}

impl<T: Real<Bool = bool> + PartialEq + PartialOrd> ScalarReal for T {}
impl<T: Float<Bool = bool> + PartialEq + PartialOrd> ScalarFloat for T {}
impl<T: Int<Bool = bool> + PartialEq + PartialOrd> ScalarInt for T {}

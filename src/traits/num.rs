use crate::simd::SimdValue;
use core::ops::{Add, Div, Mul, Rem, Sub};

pub trait Num: PartialEq + Zero + One + NumOps + SimdValue {
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

pub trait Zero: Sized {
    const ZERO: Self;
}

pub trait One: Sized {
    const ONE: Self;
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

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

pub trait NumRefOps<Rhs = Self, Output = Self>:
    NumOps<Rhs, Output> + for<'a> NumOps<&'a Rhs, Output>
{
}

pub trait RefNumOps<Rhs = Self, Output = Self>: for<'a> NumOps<&'a Rhs, Output> {}

impl<T> NumRefOps for T where T: NumOps + for<'a> NumOps<&'a T> {}
impl<T> RefNumOps for T where T: for<'a> NumOps<&'a T> {}

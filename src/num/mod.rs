#[cfg(feature = "portable_simd")]
mod portable_simd;
mod scalar;

use core::ops::{Add, Div, Mul, Rem, Sub};

pub trait Num: PartialEq + Zero + One + NumOps {}

pub trait Zero: Sized {
    const ZERO: Self;
}

pub trait One: Sized {
    const ONE: Self;
}

pub trait NegOne: Sized {
    const NEG_ONE: Self;
}

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

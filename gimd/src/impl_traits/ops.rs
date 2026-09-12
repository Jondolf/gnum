use super::{BackendMask, BackendVector};
use crate::{Mask, Simd, SimdElement};
use core::ops::*;
use gnum::num::ops::{DivEuclid, RemEuclid};

macro_rules! binary_op {
    ($trait:ident, $method:ident) => {
        impl<T: SimdElement<N>, const N: usize> $trait for Simd<T, N>
        where
            BackendVector<T, N>: $trait<Output = BackendVector<T, N>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self {
                Self::from_inner(self.0.$method(rhs.0))
            }
        }
    };
}

macro_rules! binary_ref_op {
    ($trait:ident, $method:ident) => {
        impl<'a, T: SimdElement<N>, const N: usize> $trait<&'a Self> for Simd<T, N>
        where
            BackendVector<T, N>: $trait<&'a BackendVector<T, N>, Output = BackendVector<T, N>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &'a Self) -> Self {
                Self::from_inner(self.0.$method(&rhs.0))
            }
        }
    };
}

macro_rules! assign_op {
    ($trait:ident, $method:ident) => {
        impl<T: SimdElement<N>, const N: usize> $trait for Simd<T, N>
        where
            BackendVector<T, N>: $trait,
        {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.0.$method(rhs.0);
            }
        }
    };
}

macro_rules! assign_ref_op {
    ($trait:ident, $method:ident) => {
        impl<'a, T: SimdElement<N>, const N: usize> $trait<&'a Self> for Simd<T, N>
        where
            BackendVector<T, N>: $trait<&'a BackendVector<T, N>>,
        {
            #[inline]
            fn $method(&mut self, rhs: &'a Self) {
                self.0.$method(&rhs.0);
            }
        }
    };
}

binary_op!(Add, add);
binary_op!(Sub, sub);
binary_op!(Mul, mul);
binary_op!(Div, div);
binary_op!(Rem, rem);
binary_op!(BitAnd, bitand);
binary_op!(BitOr, bitor);
binary_op!(BitXor, bitxor);
binary_op!(Shl, shl);
binary_op!(Shr, shr);
assign_op!(BitAndAssign, bitand_assign);
assign_op!(BitOrAssign, bitor_assign);
assign_op!(BitXorAssign, bitxor_assign);
assign_op!(ShlAssign, shl_assign);
assign_op!(ShrAssign, shr_assign);
assign_op!(AddAssign, add_assign);
assign_op!(SubAssign, sub_assign);
assign_op!(MulAssign, mul_assign);
assign_op!(DivAssign, div_assign);
assign_op!(RemAssign, rem_assign);
binary_ref_op!(Add, add);
binary_ref_op!(Sub, sub);
binary_ref_op!(Mul, mul);
binary_ref_op!(Div, div);
binary_ref_op!(Rem, rem);
assign_ref_op!(AddAssign, add_assign);
assign_ref_op!(SubAssign, sub_assign);
assign_ref_op!(MulAssign, mul_assign);
assign_ref_op!(DivAssign, div_assign);
assign_ref_op!(RemAssign, rem_assign);

impl<T: SimdElement<N>, const N: usize> Neg for Simd<T, N>
where
    BackendVector<T, N>: Neg<Output = BackendVector<T, N>>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::from_inner(-self.0)
    }
}

impl<T: SimdElement<N>, const N: usize> Not for Simd<T, N>
where
    BackendVector<T, N>: Not<Output = BackendVector<T, N>>,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::from_inner(!self.0)
    }
}

macro_rules! mask_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T: SimdElement<N>, const N: usize> $trait for Mask<T, N>
        where
            BackendMask<T, N>: $trait<Output = BackendMask<T, N>>,
        {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self {
                Self::from_inner(self.0 $op rhs.0)
            }
        }
    };
}

mask_op!(BitAnd, bitand, &);
mask_op!(BitOr, bitor, |);
mask_op!(BitXor, bitxor, ^);

impl<T: SimdElement<N>, const N: usize> Not for Mask<T, N>
where
    BackendMask<T, N>: Not<Output = BackendMask<T, N>>,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::from_inner(!self.0)
    }
}

impl<T: SimdElement<N>, const N: usize> DivEuclid for Simd<T, N>
where
    BackendVector<T, N>: DivEuclid<Output = BackendVector<T, N>>,
{
    type Output = Self;

    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        Self::from_inner(self.0.div_euclid(rhs.0))
    }
}

impl<T: SimdElement<N>, const N: usize> RemEuclid for Simd<T, N>
where
    BackendVector<T, N>: RemEuclid<Output = BackendVector<T, N>>,
{
    type Output = Self;

    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        Self::from_inner(self.0.rem_euclid(rhs.0))
    }
}

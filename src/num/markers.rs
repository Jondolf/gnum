use crate::{
    num::{Float, Int, Num, Real},
    simd::SimdLike,
};

/// A marker trait for scalar types.
///
/// This is used to distinguish scalar types from SIMD vector types in generic code,
/// allowing the use of methods and operators that may not be supported by SIMD vector types.
pub trait ScalarValue: PartialEq + PartialOrd {}

macro_rules! impl_scalar_value {
    ($($t:ty),*) => {
        $(
            impl ScalarValue for $t {}
        )*
    };
}

impl_scalar_value!(u8, u16, u32, u64, usize);
impl_scalar_value!(i8, i16, i32, i64, isize);
impl_scalar_value!(f32, f64);

/// A marker trait for SIMD types.
///
/// This is used to distinguish SIMD vector types from scalar types in generic code,
/// allowing the use of methods and operators that may not be supported by scalar types.
pub trait SimdValue {}

/// A marker trait for scalar [`Num`] types.
///
/// This trait is implemented by all scalar types that implement [`Num`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarNum: Num<Bool = bool> + ScalarValue {}

/// A marker trait for scalar [`Real`] types.
///
/// This trait is implemented by all scalar types that implement [`Real`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarReal: Real<Bool = bool> + ScalarValue {}

/// A marker trait for scalar [`Float`] types.
///
/// This trait is implemented by all scalar types that implement [`Float`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarFloat: Float<Bool = bool> + ScalarValue {}

/// A marker trait for scalar [`Int`] types.
///
/// This trait is implemented by all scalar types that implement [`Int`] with `Bool = bool`.
/// It allows easily constraining generic parameters to support only scalar types,
/// allowing the use of comparison operators and other methods that involve `bool` values,
/// which may not be supported by SIMD vector types.
pub trait ScalarInt: Int<Bool = bool> + ScalarValue {}

/// A marker trait for SIMD [`Num`] types.
///
/// This trait is implemented by all SIMD vector types that implement [`Num`]
/// with `Bool` being a SIMD mask type. It allows easily constraining generic parameters
/// to support only SIMD vector types.
pub trait SimdNum: Num + SimdValue {}

/// A marker trait for SIMD [`Real`] types.
///
/// This trait is implemented by all SIMD vector types that implement [`Real`]
/// with `Bool` being a SIMD mask type. It allows easily constraining generic parameters
/// to support only SIMD vector types.
pub trait SimdReal: Real + SimdValue {}

/// A marker trait for SIMD [`Float`] types.
///
/// This trait is implemented by all SIMD vector types that implement [`Float`]
/// with `Bool` being a SIMD mask type. It allows easily constraining generic parameters
/// to support only SIMD vector types.
pub trait SimdFloat: Float + SimdValue {}

/// A marker trait for SIMD [`Int`] types.
///
/// This trait is implemented by all SIMD vector types that implement [`Int`]
/// with `Bool` being a SIMD mask type. It allows easily constraining generic parameters
/// to support only SIMD vector types.
pub trait SimdInt: Int + SimdValue {}

impl<T: Num + ScalarValue + SimdLike<Bool = bool>> ScalarNum for T {}
impl<T: Real + ScalarValue + SimdLike<Bool = bool>> ScalarReal for T {}
impl<T: Float + ScalarValue + SimdLike<Bool = bool>> ScalarFloat for T {}
impl<T: Int + ScalarValue + SimdLike<Bool = bool>> ScalarInt for T {}

impl<T: Num + SimdValue> SimdNum for T {}
impl<T: Real + SimdValue> SimdReal for T {}
impl<T: Float + SimdValue> SimdFloat for T {}
impl<T: Int + SimdValue> SimdInt for T {}

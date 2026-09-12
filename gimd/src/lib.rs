#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "portable_simd", feature(portable_simd))]
#![warn(missing_docs)]

#[cfg(not(any(feature = "wide", feature = "portable_simd")))]
compile_error!("gimd requires either the `wide` or `portable_simd` feature");

mod backend;
mod features;
mod impl_traits;

use core::{
    fmt,
    ops::{Index, IndexMut},
};

#[doc(hidden)]
pub use backend::SimdElement;

/// A SIMD vector containing `N` elements of type `T`.
///
/// Most numeric operations and lane manipulation methods are implemented
/// via traits from the [`gnum`] crate.
#[repr(transparent)]
pub struct Simd<T: SimdElement<N>, const N: usize>(<T as SimdElement<N>>::Vector);

/// A SIMD vector mask associated with [`Simd<T, N>`].
///
/// Most boolean operations and lane manipulation methods are implemented
/// via traits from the [`gnum`] crate.
#[repr(transparent)]
pub struct Mask<T: SimdElement<N>, const N: usize>(<T as SimdElement<N>>::Mask);

impl<T: SimdElement<N>, const N: usize> Simd<T, N> {
    /// Creates a new SIMD vector with all elements set to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::u32x4;
    /// let v = u32x4::splat(8);
    /// assert_eq!(v.as_array(), &[8, 8, 8, 8]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(value: T) -> Self {
        Self::from_array([value; N])
    }

    /// Creates a new SIMD vector from an array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::u32x4;
    /// let v = u32x4::from_array([1, 2, 3, 4]);
    /// assert_eq!(v.as_array(), &[1, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Self {
        assert!(core::mem::size_of::<[T; N]>() == core::mem::size_of::<T::Vector>());

        // SAFETY: `SimdElement` is sealed and every implementation maps to
        //         a backend vector with the same lane representation as `[T; N]`.
        Self(unsafe { core::mem::transmute_copy(&array) })
    }

    /// Converts the SIMD vector to an array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::u32x4;
    /// let v = u32x4::from_array([1, 2, 3, 4]);
    /// assert_eq!(v.to_array(), [1, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_array(self) -> [T; N] {
        gnum::simd::SimdLike::to_array(self.0)
    }

    /// Returns an array reference containing the entire SIMD vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::u32x4;
    /// let v = u32x4::from_array([1, 2, 3, 4]);
    /// assert_eq!(v.as_array(), &[1, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_array(&self) -> &[T; N] {
        T::vector_as_array(&self.0)
    }

    /// Returns a mutable array reference containing the entire SIMD vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::u32x4;
    /// let mut v = u32x4::from_array([1, 2, 3, 4]);
    /// let a = v.as_mut_array();
    /// a[0] = 10;
    /// assert_eq!(v.as_array(), &[10, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        T::vector_as_mut_array(&mut self.0)
    }

    /// Creates a new [`Simd<T, N>`] from the underlying SIMD vector type.
    #[inline]
    #[must_use]
    pub const fn from_inner(inner: <T as SimdElement<N>>::Vector) -> Self {
        Self(inner)
    }

    /// Returns the underlying SIMD vector type.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> <T as SimdElement<N>>::Vector {
        self.0
    }

    /// Casts each lane to `U` with Rust's `as` semantics.
    ///
    /// See [`gnum::num::NumCast`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{f32x4, u32x4};
    /// let v = u32x4::from_array([1, 2, 3, 4]);
    /// let cast: f32x4 = v.cast();
    /// assert_eq!(cast.to_array(), [1.0, 2.0, 3.0, 4.0]);
    /// ```
    #[inline]
    #[must_use]
    pub fn cast<U: SimdElement<N>>(self) -> Simd<U, N>
    where
        Self: gnum::num::NumCast<Simd<U, N>>,
    {
        gnum::num::NumCast::cast(self)
    }
}

impl<T: SimdElement<N>, const N: usize> Mask<T, N> {
    /// A mask with every lane set to `true`.
    pub const TRUE: Self = Self(<T::Mask as gnum::simd::MaskLike>::TRUE);

    /// A mask with every lane set to `false`.
    pub const FALSE: Self = Self(<T::Mask as gnum::simd::MaskLike>::FALSE);

    /// Creates a new mask with all elements set to the given boolean value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let mask = Mask::<f32, 4>::splat(true);
    /// assert_eq!(mask.to_array(), [true, true, true, true]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(value: bool) -> Self {
        if value { Self::TRUE } else { Self::FALSE }
    }

    /// Creates a new mask from an array of booleans.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.to_array(), lanes);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_array(array: [bool; N]) -> Self {
        Self(T::mask_from_array(array))
    }

    /// Converts the mask to an array of booleans.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.to_array(), lanes);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_array(self) -> [bool; N] {
        T::mask_to_array(self.0)
    }

    /// Creates a new [`Mask<T, N>`] from the underlying mask type.
    #[inline]
    #[must_use]
    pub const fn from_inner(inner: <T as SimdElement<N>>::Mask) -> Self {
        Self(inner)
    }

    /// Returns the underlying mask type.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> <T as SimdElement<N>>::Mask {
        self.0
    }

    /// Tests the boolean value of the specified element.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.test(0), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        gnum::simd::MaskLike::test(&self.0, index)
    }

    /// Tests the boolean value of the specified element without bounds checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `index` is greater than or equal to the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// unsafe {
    ///     assert_eq!(mask.test_unchecked(0), true);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub unsafe fn test_unchecked(&self, index: usize) -> bool {
        unsafe { gnum::simd::MaskLike::test_unchecked(&self.0, index) }
    }

    /// Sets the boolean value of the specified element.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mut mask = Mask::<f32, 4>::from_array(lanes);
    /// mask.set(0, false);
    /// assert_eq!(mask.to_array(), [false, false, false, true]);
    /// ```
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        gnum::simd::MaskLike::set(&mut self.0, index, value);
    }

    /// Sets the boolean value of the specified element without bounds checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `index` is greater than or equal to the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mut mask = Mask::<f32, 4>::from_array(lanes);
    /// unsafe {
    ///     mask.set_unchecked(0, false);
    /// }
    /// assert_eq!(mask.to_array(), [false, false, false, true]);
    /// ```
    #[inline]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        unsafe { gnum::simd::MaskLike::set_unchecked(&mut self.0, index, value) }
    }

    /// Returns a bitmask representing the boolean state of each mask element.
    ///
    /// Set bits in the result correspond to `true` values in the mask,
    /// and unset bits correspond to `false` values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.to_bitmask(), 0b1001);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_bitmask(self) -> u64 {
        gnum::simd::MaskLike::to_bitmask(self.0)
    }

    /// Returns `true` if all elements in the mask are `true`, or `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.all(), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        gnum::simd::MaskLike::all(self.0)
    }

    /// Returns `true` if any element in the mask is `true`, or `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// assert_eq!(mask.any(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        gnum::simd::MaskLike::any(self.0)
    }

    /// Chooses elements from two SIMD vectors using the mask.
    ///
    /// For each element in the mask, chooses the corresponding element from `true_values` if
    /// that element mask is `true`, and from `false_values` if that element mask is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gimd::{Mask, f32x4};
    /// let lanes = [true, false, false, true];
    /// let mask = Mask::<f32, 4>::from_array(lanes);
    /// let true_values = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    /// let false_values = f32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    /// let result = mask.select(true_values, false_values);
    /// assert_eq!(result.to_array(), [1.0, 6.0, 7.0, 4.0]);
    /// ```
    #[inline]
    #[must_use]
    pub fn select<V>(self, true_values: V, false_values: V) -> V
    where
        Self: gnum::simd::Select<V>,
    {
        gnum::simd::Select::select(self, true_values, false_values)
    }
}

impl<T: SimdElement<N>, const N: usize> Copy for Simd<T, N> {}

impl<T: SimdElement<N>, const N: usize> Clone for Simd<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: SimdElement<N>, const N: usize> Copy for Mask<T, N> {}

impl<T: SimdElement<N>, const N: usize> Clone for Mask<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: SimdElement<N>, const N: usize> fmt::Debug for Simd<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T; N] as fmt::Debug>::fmt(self.as_array(), f)
    }
}

impl<T: SimdElement<N>, const N: usize> fmt::Debug for Mask<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_array().fmt(f)
    }
}

impl<T: SimdElement<N> + PartialEq, const N: usize> PartialEq for Simd<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_array() == other.as_array()
    }
}

impl<T: SimdElement<N> + Eq, const N: usize> Eq for Simd<T, N> {}

impl<T: SimdElement<N>, const N: usize> PartialEq for Mask<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.to_bitmask() == other.to_bitmask()
    }
}

impl<T: SimdElement<N>, const N: usize> Eq for Mask<T, N> {}

impl<T: SimdElement<N>, const N: usize> Index<usize> for Simd<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array()[index]
    }
}

impl<T: SimdElement<N>, const N: usize> IndexMut<usize> for Simd<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_array()[index]
    }
}

impl<T: SimdElement<N>, const N: usize> From<[T; N]> for Simd<T, N> {
    fn from(value: [T; N]) -> Self {
        Self::from_array(value)
    }
}

impl<T: SimdElement<N>, const N: usize> From<Simd<T, N>> for [T; N] {
    fn from(value: Simd<T, N>) -> Self {
        value.to_array()
    }
}

impl<T: SimdElement<N>, const N: usize> From<[bool; N]> for Mask<T, N> {
    fn from(value: [bool; N]) -> Self {
        Self::from_array(value)
    }
}

impl<T: SimdElement<N>, const N: usize> From<Mask<T, N>> for [bool; N] {
    fn from(value: Mask<T, N>) -> Self {
        value.to_array()
    }
}

impl<T: SimdElement<N>, const N: usize> AsRef<[T; N]> for Simd<T, N> {
    fn as_ref(&self) -> &[T; N] {
        self.as_array()
    }
}

impl<T: SimdElement<N>, const N: usize> AsMut<[T; N]> for Simd<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        self.as_mut_array()
    }
}

macro_rules! aliases {
    ($($name:ident = $element:ty, $lanes:literal);* $(;)?) => {
        $(
            #[doc = concat!("A SIMD vector with `", stringify!($lanes), "` `", stringify!($element), "` lanes.")]
            #[allow(non_camel_case_types)]
            pub type $name = Simd<$element, $lanes>;
        )*
    };
}

aliases! {
    f32x4 = f32, 4; f32x8 = f32, 8; f32x16 = f32, 16;
    f64x2 = f64, 2; f64x4 = f64, 4; f64x8 = f64, 8;
    i8x16 = i8, 16; i8x32 = i8, 32;
    i16x8 = i16, 8; i16x16 = i16, 16; i16x32 = i16, 32;
    i32x4 = i32, 4; i32x8 = i32, 8; i32x16 = i32, 16;
    i64x2 = i64, 2; i64x4 = i64, 4; i64x8 = i64, 8;
    u8x16 = u8, 16; u8x32 = u8, 32;
    u16x8 = u16, 8; u16x16 = u16, 16; u16x32 = u16, 32;
    u32x4 = u32, 4; u32x8 = u32, 8; u32x16 = u32, 16;
    u64x2 = u64, 2; u64x4 = u64, 4; u64x8 = u64, 8;
}

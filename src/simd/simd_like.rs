use super::{MaskLike, Select};

/// Base trait for SIMD-like types.
///
/// This trait is implemented by both scalar types and SIMD vector types. It is designed to be used
/// in generic code that can work with both scalar and SIMD types in an [AoSoA] setting.
///
/// [AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA
pub trait SimdLike: Sized {
    /// The number of lanes of this SIMD value.
    const LANES: usize;

    /// The type of the elements of each lane of this SIMD value.
    type Element: SimdLike<Element = Self::Element, Bool = bool>;

    /// Type of the result of comparing two SIMD values like `self`.
    type Bool: MaskLike + Select<Self>;

    /// Initializes an SIMD value with each lanes set to `val`.
    fn splat(val: Self::Element) -> Self;

    /// Extracts the i-th lane of `self`.
    ///
    /// # Panics
    ///
    /// Panics if `i >= Self::LANES`.
    fn extract(&self, i: usize) -> Self::Element;

    /// Extracts the i-th lane of `self` without bound-checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `i >= Self::LANES`.
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element;

    /// Replaces the i-th lane of `self` by `val`.
    ///
    /// Panics if `i >= Self::LANES`.
    fn replace(&mut self, i: usize, val: Self::Element);

    /// Replaces the i-th lane of `self` by `val` without bound-checking.
    ///
    /// # Safety
    ///
    /// Undefined behavior if `i >= Self::LANES`.
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element);

    /// Applies a function to each lane of `self`.
    ///
    /// Note that, while convenient, this method can be extremely slow as this
    /// requires to extract each lane of `self` and then combine them again into
    /// a new SIMD value.
    #[inline(always)]
    fn map_lanes(self, f: impl Fn(Self::Element) -> Self::Element) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();

        for i in 0..Self::LANES {
            unsafe { result.replace_unchecked(i, f(self.extract_unchecked(i))) }
        }

        result
    }

    /// Applies a function to each lane of `self` paired with the corresponding lane of `b`.
    ///
    /// Note that, while convenient, this method can be extremely slow as this
    /// requires to extract each lane of `self` and then combine them again into
    /// a new SIMD value.
    #[inline(always)]
    fn zip_map_lanes(
        self,
        b: Self,
        f: impl Fn(Self::Element, Self::Element) -> Self::Element,
    ) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();

        for i in 0..Self::LANES {
            unsafe {
                let a = self.extract_unchecked(i);
                let b = b.extract_unchecked(i);
                result.replace_unchecked(i, f(a, b))
            }
        }

        result
    }
}

macro_rules! impl_simd_value_scalar {
    ($($t:ty),*) => {
        $(
            impl SimdLike for $t {
                const LANES: usize = 1;
                type Element = Self;
                type Bool = bool;

                #[inline]
                fn splat(val: Self::Element) -> Self {
                    val
                }

                #[inline]
                fn extract(&self, i: usize) -> Self::Element {
                    debug_assert!(i == 0, "index out of bounds");
                    *self
                }

                #[inline]
                unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                    debug_assert!(i == 0, "index out of bounds");
                    *self
                }

                #[inline]
                fn replace(&mut self, i: usize, val: Self::Element) {
                    debug_assert!(i == 0, "index out of bounds");
                    *self = val;
                }

                #[inline]
                unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                    debug_assert!(i == 0, "index out of bounds");
                    *self = val;
                }
            }
        )*
    };
}

impl_simd_value_scalar!(u8, u16, u32, u64, usize);
impl_simd_value_scalar!(i8, i16, i32, i64, isize);
impl_simd_value_scalar!(f32, f64);

impl SimdLike for bool {
    const LANES: usize = 1;
    type Element = Self;
    type Bool = Self;

    #[inline]
    fn splat(val: Self::Element) -> Self {
        val
    }

    #[inline]
    fn extract(&self, i: usize) -> Self::Element {
        debug_assert!(i == 0, "index out of bounds");
        *self
    }

    #[inline]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        debug_assert!(i == 0, "index out of bounds");
        *self
    }

    #[inline]
    fn replace(&mut self, i: usize, val: Self::Element) {
        debug_assert!(i == 0, "index out of bounds");
        *self = val;
    }

    #[inline]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        debug_assert!(i == 0, "index out of bounds");
        *self = val;
    }
}

use super::MaskLike;

/// Choose elements from two vectors using a mask.
///
/// For each element in the mask, choose the corresponding element from `true_values` if
/// that element mask is true, and `false_values` if that element mask is false.
pub trait Select<T>: MaskLike {
    /// Choose elements from two vectors.
    ///
    /// For each element in the mask, choose the corresponding element from `true_values` if
    /// that element mask is true, and `false_values` if that element mask is false.
    fn select(self, true_values: T, false_values: T) -> T;
}

impl<T> Select<T> for bool {
    #[inline]
    fn select(self, true_values: T, false_values: T) -> T {
        if self { true_values } else { false_values }
    }
}

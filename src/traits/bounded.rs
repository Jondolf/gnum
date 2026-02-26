/// A trait for types that have a well-defined minimum and maximum value.
pub trait Bounded {
    /// The smallest finite value that can be represented by this type.
    const MIN: Self;
    /// The largest finite value that can be represented by this type.
    const MAX: Self;
}

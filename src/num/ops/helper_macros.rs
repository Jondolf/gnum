/// Implements a unary operator trait for the specified types,
/// allowing for both owned and reference operands.
macro_rules! impl_unary_op {
    ($trait_name:ident, $method:ident, $($t:ty),+) => {
        $(
            impl $trait_name for $t {
                #[inline]
                fn $method(self) -> Self::Output {
                    <$t>::$method(self)
                }
            }

            impl $trait_name for &$t {
                #[inline]
                fn $method(self) -> Self::Output {
                    <$t>::$method(*self)
                }
            }
        )+
    };
}

/// Implements a binary operator trait for the specified types,
/// allowing for both owned and reference operands.
macro_rules! impl_binary_op {
    ($trait_name:ident, $method:ident, $($t:ty),+) => {
        $(
            impl $trait_name for $t {
                #[inline]
                fn $method(self, rhs: Self) -> Self::Output {
                    <$t>::$method(self, rhs)
                }
            }

            impl $trait_name<&$t> for $t {
                #[inline]
                fn $method(self, rhs: &Self) -> Self::Output {
                    <$t>::$method(self, *rhs)
                }
            }

            impl $trait_name<$t> for &$t {
                #[inline]
                fn $method(self, rhs: $t) -> Self::Output {
                    <$t>::$method(*self, rhs)
                }
            }

            impl $trait_name<&$t> for &$t {
                #[inline]
                fn $method(self, rhs: &$t) -> Self::Output {
                    <$t>::$method(*self, *rhs)
                }
            }
        )+
    };
}

pub(crate) use {impl_binary_op, impl_unary_op};

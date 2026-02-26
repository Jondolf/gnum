use crate::traits::{Num, One, Zero};

macro_rules! impl_num_scalar {
    ($($t:ty),*) => {
        $(
            impl Num for $t {}
        )*
    };
}

impl_num_scalar!(u8, u16, u32, u64, usize);
impl_num_scalar!(i8, i16, i32, i64, isize);
impl_num_scalar!(f32, f64);

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

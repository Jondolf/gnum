mod bytes;
mod cmp;
mod float;
mod int;
mod num;
mod ops;
mod select;
mod signed;
mod simd_bool;
mod simd_value;

pub use cmp::{NumEq, NumOrd};
pub use select::Select;
pub use simd_bool::SimdBool;
pub use simd_value::SimdValue;

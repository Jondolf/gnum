use super::{NegOne, One, Zero};

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl One for f32 {
    const ONE: Self = 1.0;
}

impl NegOne for f32 {
    const NEG_ONE: Self = -1.0;
}

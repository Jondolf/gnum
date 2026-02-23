mod scalar;

use core::num::FpCategory;

pub trait FloatClassify {
    fn classify(self) -> FpCategory;
}

mod scalar;

use core::cmp::Ordering;

pub trait TotalCmp {
    fn total_cmp(&self, other: &Self) -> Ordering;
}

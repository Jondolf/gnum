//! Implementations of `gnum` traits for [`wide`] types.

/// Applies a scalar function to matching lanes of `$a` and `$b`, returning the same vector type.
macro_rules! zip_lanes {
    ($simd:ident, $a:expr, $b:expr, $f:expr) => {{
        let a = $a.to_array();
        let b = $b.to_array();
        let mut r = a;
        for i in 0..r.len() {
            r[i] = ($f)(a[i], b[i]);
        }
        <$simd>::new(r)
    }};
}

/// Applies a scalar function to each lane of `$self`, returning the `$out` vector type.
macro_rules! map_lanes_to {
    ($out:ident, $self:expr, $f:expr) => {{ <$out>::new($self.to_array().map($f)) }};
}

/// Applies a scalar function to matching lanes of `$a` and `$b`, returning the `$out` vector type.
macro_rules! zip_lanes_to {
    ($out:ident, $a:expr, $b:expr, $f:expr) => {{
        let b = $b.to_array();
        let mut i = 0usize;
        let r = $a.to_array().map(|a| {
            let v = ($f)(a, b[i]);
            i += 1;
            v
        });
        <$out>::new(r)
    }};
}

mod cast;
mod cmp;
mod float;
mod int;
mod markers;
mod num;
mod ops;
mod round;
mod signed;
mod simd;

use core::simd::{Simd, SimdElement, simd_swizzle};

use crate::simd::{Swizzle, TransposeRows, blocked_transpose, generic_transpose};

#[inline]
fn transpose4<T: SimdElement>(rows: [Simd<T, 4>; 4]) -> [Simd<T, 4>; 4] {
    let [b0, b1, b2, b3] = rows;

    let t0 = simd_swizzle!(b0, b1, [0, 4, 1, 5]);
    let t1 = simd_swizzle!(b0, b1, [2, 6, 3, 7]);
    let t2 = simd_swizzle!(b2, b3, [0, 4, 1, 5]);
    let t3 = simd_swizzle!(b2, b3, [2, 6, 3, 7]);

    [
        simd_swizzle!(t0, t2, [0, 1, 4, 5]),
        simd_swizzle!(t0, t2, [2, 3, 6, 7]),
        simd_swizzle!(t1, t3, [0, 1, 4, 5]),
        simd_swizzle!(t1, t3, [2, 3, 6, 7]),
    ]
}

#[inline]
fn transpose8<T: SimdElement>(rows: [Simd<T, 8>; 8]) -> [Simd<T, 8>; 8] {
    let [b0, b1, b2, b3, b4, b5, b6, b7] = rows;

    let t0 = simd_swizzle!(b0, b1, [0, 8, 1, 9, 4, 12, 5, 13]);
    let t1 = simd_swizzle!(b0, b1, [2, 10, 3, 11, 6, 14, 7, 15]);
    let t2 = simd_swizzle!(b2, b3, [0, 8, 1, 9, 4, 12, 5, 13]);
    let t3 = simd_swizzle!(b2, b3, [2, 10, 3, 11, 6, 14, 7, 15]);
    let t4 = simd_swizzle!(b4, b5, [0, 8, 1, 9, 4, 12, 5, 13]);
    let t5 = simd_swizzle!(b4, b5, [2, 10, 3, 11, 6, 14, 7, 15]);
    let t6 = simd_swizzle!(b6, b7, [0, 8, 1, 9, 4, 12, 5, 13]);
    let t7 = simd_swizzle!(b6, b7, [2, 10, 3, 11, 6, 14, 7, 15]);

    let u0 = simd_swizzle!(t0, t2, [0, 1, 8, 9, 4, 5, 12, 13]);
    let u1 = simd_swizzle!(t0, t2, [2, 3, 10, 11, 6, 7, 14, 15]);
    let u2 = simd_swizzle!(t1, t3, [0, 1, 8, 9, 4, 5, 12, 13]);
    let u3 = simd_swizzle!(t1, t3, [2, 3, 10, 11, 6, 7, 14, 15]);
    let u4 = simd_swizzle!(t4, t6, [0, 1, 8, 9, 4, 5, 12, 13]);
    let u5 = simd_swizzle!(t4, t6, [2, 3, 10, 11, 6, 7, 14, 15]);
    let u6 = simd_swizzle!(t5, t7, [0, 1, 8, 9, 4, 5, 12, 13]);
    let u7 = simd_swizzle!(t5, t7, [2, 3, 10, 11, 6, 7, 14, 15]);

    [
        simd_swizzle!(u0, u4, [0, 1, 2, 3, 8, 9, 10, 11]),
        simd_swizzle!(u1, u5, [0, 1, 2, 3, 8, 9, 10, 11]),
        simd_swizzle!(u2, u6, [0, 1, 2, 3, 8, 9, 10, 11]),
        simd_swizzle!(u3, u7, [0, 1, 2, 3, 8, 9, 10, 11]),
        simd_swizzle!(u0, u4, [4, 5, 6, 7, 12, 13, 14, 15]),
        simd_swizzle!(u1, u5, [4, 5, 6, 7, 12, 13, 14, 15]),
        simd_swizzle!(u2, u6, [4, 5, 6, 7, 12, 13, 14, 15]),
        simd_swizzle!(u3, u7, [4, 5, 6, 7, 12, 13, 14, 15]),
    ]
}

#[inline(always)]
fn blocked<T: SimdElement, const N: usize, const BN: usize, const K: usize>(
    rows: [Simd<T, N>; N],
    transpose_block: impl Fn([Simd<T, BN>; BN]) -> [Simd<T, BN>; BN],
) -> [Simd<T, N>; N] {
    blocked_transpose::<Simd<T, N>, Simd<T, BN>, N, BN, K>(
        rows,
        |row| {
            let lanes = row.to_array();
            core::array::from_fn(|block| {
                Simd::from_array(core::array::from_fn(|i| lanes[block * BN + i]))
            })
        },
        |blocks| Simd::from_array(core::array::from_fn(|i| blocks[i / BN][i % BN])),
        transpose_block,
    )
}

macro_rules! impl_generic {
    ($($lanes:literal),+ $(,)?) => {
        $(
            impl<T: SimdElement> TransposeRows<$lanes> for Simd<T, $lanes>
            where
                Simd<T, $lanes>: Swizzle,
            {
                #[inline]
                fn transpose_rows(rows: [Self; $lanes]) -> [Self; $lanes] {
                    generic_transpose(rows)
                }
            }
        )+
    };
}

impl_generic!(1, 2, 32, 64);

impl<T: SimdElement> TransposeRows<4> for Simd<T, 4>
where
    Simd<T, 4>: Swizzle,
{
    #[inline]
    fn transpose_rows(rows: [Self; 4]) -> [Self; 4] {
        transpose4(rows)
    }
}

impl<T: SimdElement> TransposeRows<8> for Simd<T, 8>
where
    Simd<T, 8>: Swizzle,
{
    #[inline]
    fn transpose_rows(rows: [Self; 8]) -> [Self; 8] {
        if size_of::<T>() == 1 {
            return generic_transpose(rows);
        }
        #[cfg(not(target_feature = "avx"))]
        if size_of::<T>() == 2 {
            return generic_transpose(rows);
        }
        transpose8(rows)
    }
}

impl<T: SimdElement> TransposeRows<16> for Simd<T, 16>
where
    Simd<T, 16>: Swizzle,
{
    #[inline]
    fn transpose_rows(rows: [Self; 16]) -> [Self; 16] {
        if size_of::<T>() <= 2 {
            return generic_transpose(rows);
        }
        blocked::<T, 16, 8, 2>(rows, transpose8)
    }
}

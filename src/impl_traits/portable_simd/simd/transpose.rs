use core::simd::{Simd, SimdElement, simd_swizzle};

use crate::simd::{Swizzle, Transpose, generic_transpose};

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
fn blocked_transpose<T: SimdElement, const N: usize, const BN: usize, const K: usize>(
    rows: [Simd<T, N>; N],
    transpose_block: impl Fn([Simd<T, BN>; BN]) -> [Simd<T, BN>; BN],
) -> [Simd<T, N>; N] {
    let row_blocks: [[Simd<T, BN>; K]; N] = rows.map(|row| {
        let lanes = row.to_array();
        core::array::from_fn(|block| {
            Simd::from_array(core::array::from_fn(|i| lanes[block * BN + i]))
        })
    });
    let mut column_blocks = [[row_blocks[0][0]; K]; N];

    let mut block_row = 0;
    while block_row < K {
        let mut block_column = 0;
        while block_column < K {
            let block = core::array::from_fn(|r| row_blocks[block_row * BN + r][block_column]);
            let block = transpose_block(block);
            let mut c = 0;
            while c < BN {
                column_blocks[block_column * BN + c][block_row] = block[c];
                c += 1;
            }
            block_column += 1;
        }
        block_row += 1;
    }

    column_blocks.map(|blocks| Simd::from_array(core::array::from_fn(|i| blocks[i / BN][i % BN])))
}

macro_rules! impl_generic {
    ($($lanes:literal),+ $(,)?) => {
        $(
            impl<T: SimdElement> Transpose for [Simd<T, $lanes>; $lanes]
            where
                Simd<T, $lanes>: Swizzle,
            {
                #[inline]
                fn transpose(self) -> Self {
                    generic_transpose(self)
                }
            }
        )+
    };
}

impl_generic!(1, 2, 32, 64);

impl<T: SimdElement> Transpose for [Simd<T, 4>; 4]
where
    Simd<T, 4>: Swizzle,
{
    #[inline]
    fn transpose(self) -> Self {
        transpose4(self)
    }
}

impl<T: SimdElement> Transpose for [Simd<T, 8>; 8]
where
    Simd<T, 8>: Swizzle,
{
    #[inline]
    fn transpose(self) -> Self {
        if size_of::<T>() == 1 {
            return generic_transpose(self);
        }
        #[cfg(not(target_feature = "avx"))]
        if size_of::<T>() == 2 {
            return generic_transpose(self);
        }
        transpose8(self)
    }
}

impl<T: SimdElement> Transpose for [Simd<T, 16>; 16]
where
    Simd<T, 16>: Swizzle,
{
    #[inline]
    fn transpose(self) -> Self {
        if size_of::<T>() <= 2 {
            return generic_transpose(self);
        }
        blocked_transpose::<T, 16, 8, 2>(self, transpose8)
    }
}

use super::{RealElement, SimdElement, sealed::Sealed};

macro_rules! simd_element {
    ($($element:ty, $lanes:literal => $vector:ident);* $(;)?) => {
        $(
            impl Sealed<$lanes> for $element {}

            impl SimdElement<$lanes> for $element {
                type Vector = wide::$vector;
                type MaskInt = <wide::$vector as gnum::simd::MaskCast>::Int;
                type Mask = wide::$vector;

                #[inline]
                fn vector_as_array(vector: &Self::Vector) -> &[Self; $lanes] {
                    vector.as_array()
                }

                #[inline]
                fn vector_as_mut_array(vector: &mut Self::Vector) -> &mut [Self; $lanes] {
                    vector.as_mut_array()
                }
            }
        )*
    };
}

simd_element! {
    f32, 4 => f32x4; f32, 8 => f32x8; f32, 16 => f32x16;
    f64, 2 => f64x2; f64, 4 => f64x4; f64, 8 => f64x8;
    i8, 16 => i8x16; i8, 32 => i8x32;
    i16, 8 => i16x8; i16, 16 => i16x16; i16, 32 => i16x32;
    i32, 4 => i32x4; i32, 8 => i32x8; i32, 16 => i32x16;
    i64, 2 => i64x2; i64, 4 => i64x4; i64, 8 => i64x8;
    u8, 16 => u8x16; u8, 32 => u8x32;
    u16, 8 => u16x8; u16, 16 => u16x16; u16, 32 => u16x32;
    u32, 4 => u32x4; u32, 8 => u32x8; u32, 16 => u32x16;
    u64, 2 => u64x2; u64, 4 => u64x4; u64, 8 => u64x8;
}

macro_rules! real_elements {
    ($($element:ty, $lanes:literal => $i32:ty);* $(;)?) => {
        $(
            impl RealElement<$lanes> for $element {
                type I32Element = $i32;
            }
        )*
    };
}

real_elements! {
    f32, 4 => i32; f32, 8 => i32; f32, 16 => i32;
    f64, 2 => i64; f64, 4 => i64; f64, 8 => i64;
}

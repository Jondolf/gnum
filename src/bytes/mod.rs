#[cfg(feature = "portable_simd")]
mod portable_simd;
mod scalar;

pub trait ToBytes {
    type Bytes: Copy + Unpin + Send + Sync + AsRef<[u8]> + AsMut<[u8]> + 'static;

    fn to_ne_bytes(self) -> Self::Bytes;
    fn to_be_bytes(self) -> Self::Bytes;
    fn to_le_bytes(self) -> Self::Bytes;
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

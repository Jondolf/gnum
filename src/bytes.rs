/// Converts values to and from their byte representations.
pub trait ToBytes {
    /// This type, reinterpreted as bytes.
    type Bytes: Copy + Unpin + Send + Sync + AsRef<[u8]> + AsMut<[u8]> + 'static;

    /// Returns the memory representation of `self` as a byte array in native byte order.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_ne_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of `self` as a byte array in big-endian (network) byte order.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_be_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of `self` as a byte array in little-endian byte order.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn to_le_bytes(self) -> Self::Bytes;

    /// Creates a native endian value from its memory representation as a byte array in native endianness.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a value from its representation as a byte array in big endian.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Creates a value from its representation as a byte array in little endian.s
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

macro_rules! impl_to_bytes {
    ($($t:ty => $bytes:expr),*) => {
        $(
            impl ToBytes for $t {
                type Bytes = [u8; $bytes];

                #[inline]
                fn to_ne_bytes(self) -> [u8; $bytes] {
                    self.to_ne_bytes()
                }
                #[inline]
                fn to_be_bytes(self) -> [u8; $bytes] {
                    self.to_be_bytes()
                }
                #[inline]
                fn to_le_bytes(self) -> [u8; $bytes] {
                    self.to_le_bytes()
                }
                #[inline]
                fn from_ne_bytes(bytes: [u8; $bytes]) -> Self {
                    Self::from_ne_bytes(bytes)
                }
                #[inline]
                fn from_be_bytes(bytes: [u8; $bytes]) -> Self {
                    Self::from_be_bytes(bytes)
                }
                #[inline]
                fn from_le_bytes(bytes: [u8; $bytes]) -> Self {
                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

impl_to_bytes!(
    u8 => 1,
    u16 => 2,
    u32 => 4,
    u64 => 8,
    u128 => 16,
    usize => core::mem::size_of::<usize>(),
    i8 => 1,
    i16 => 2,
    i32 => 4,
    i64 => 8,
    i128 => 16,
    isize => core::mem::size_of::<isize>(),
    f32 => 4,
    f64 => 8
);

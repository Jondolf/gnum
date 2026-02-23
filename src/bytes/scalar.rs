use super::ToBytes;

impl ToBytes for f32 {
    type Bytes = [u8; 4];

    fn to_ne_bytes(self) -> [u8; 4] {
        self.to_ne_bytes()
    }
    fn to_be_bytes(self) -> [u8; 4] {
        self.to_be_bytes()
    }
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_le_bytes()
    }
    fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self::from_ne_bytes(bytes)
    }
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self::from_be_bytes(bytes)
    }
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

impl ToBytes for f64 {
    type Bytes = [u8; 8];

    fn to_ne_bytes(self) -> [u8; 8] {
        self.to_ne_bytes()
    }
    fn to_be_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_le_bytes()
    }
    fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self::from_ne_bytes(bytes)
    }
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum InvalidBytecode {
    NotEnoughBytes(u8),
    InvalidByte(u8)
}
impl Display for InvalidBytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughBytes(last_byte) => write!(f, "Expected byte after {}, found EOF", last_byte),
            Self::InvalidByte(byte) => write!(f, "Invalid byte: {}", byte),
        }
    }
}
impl Error for InvalidBytecode {}
impl InvalidBytecode {
    pub fn expected_byte(last_byte: u8) -> Self {
        Self::NotEnoughBytes(last_byte)
    }
    pub fn unknown_byte(byte: u8) -> Self {
        Self::InvalidByte(byte)
    }
}
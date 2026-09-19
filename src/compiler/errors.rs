use std::fmt;
use std::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct InvalidBytecode {
    byte: u8,
}
impl fmt::Display for InvalidBytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "Invalid byte in bytecode: 0x{:x}", self.byte
        )
    }
}
impl Error for InvalidBytecode {}

#[derive(Clone, Copy, Debug)]
pub struct CannotInline {}
impl fmt::Display for CannotInline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This value cannot be determined at compile-time")
    }
}
impl Error for CannotInline {}
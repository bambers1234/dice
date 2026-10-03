use std::{error::Error, fmt::Display};

use crate::vm::bytecode::Bytecode;

#[derive(Clone, Copy, Debug)]
pub enum InvalidBytecode {
    NotEnoughBytes,
    InvalidByte(u8),
    NotEnoughOnStack{op: super::bytecode::Bytecode}
}
impl Display for InvalidBytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughBytes => write!(f, "Expected byte, found EOF."),
            Self::InvalidByte(byte) => write!(f, "Invalid byte: {}.", byte),
            Self::NotEnoughOnStack { op } => write!(f, "Not enough operands on stack for operator {}", op)
        }
    }
}
impl Error for InvalidBytecode {}
impl InvalidBytecode {
    pub fn expected_byte() -> Self {
        Self::NotEnoughBytes
    }
    pub fn unknown_byte(byte: u8) -> Self {
        Self::InvalidByte(byte)
    }
    pub fn not_enough_on_stack(op: Bytecode) -> Self {
        Self::NotEnoughOnStack { op }
    }
}
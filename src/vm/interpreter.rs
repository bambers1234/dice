use std::sync::RwLock;
use crate::vm::bytecode::Bytecode;

use super::errors::InvalidBytecode;

static STACK: RwLock<Vec<u8>> = RwLock::new(Vec::new());

/// An iterator over raw bytes that are not necessarily bytecode
#[derive(Debug, Clone, Copy)]
pub struct Bytes<'a> {
    bytes: &'a [u8],
    index: usize,
}
impl<'a> Iterator for Bytes<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.bytes.len() {
            return None
        };
        self.index += 1;
        Some(self.bytes[self.index - 1])
    }
}
impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            bytes: value,
            index: 0
        }
    }
}

/// Receive a byte stream which may or may not be valid bytecode and execute it. 
/// ## Errors: Invalid bytecode
pub fn execute(mut bytecode: Bytes) -> Result<(), InvalidBytecode> {
    let mut stack = STACK.write().expect("There has been a panic in the interpreter, causing the memory stack to poison.\nThis is a bug in the interpreter.");
    while let Some(byte) = bytecode.next() {
        let byte = Bytecode::try_from(byte)?;
        byte.execute(&mut *stack, &mut bytecode)?;
    }
    Ok(())
}

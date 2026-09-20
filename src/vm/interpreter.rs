use std::sync::RwLock;

use super::bytecode::Bytecode::{self, *};
use super::errors::InvalidBytecode;

static STACK: RwLock<Vec<u16>> = RwLock::new(Vec::new());

#[derive(Debug, Clone)]
struct Cursor {
    bytes: std::vec::IntoIter<u8>,
}
impl Iterator for Cursor {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.next()
    }
}
impl Cursor {
    fn next_instruction(&mut self, stack: &mut Vec<u16>) -> Result<(), InvalidBytecode>{
        let next = self.next();
        if next.is_none() {
            todo!("End of bytecode")
        }
        #[allow(clippy::unwrap_used)]
        let instruction = next.unwrap();
        let code = Bytecode::try_from(instruction)?;
        match code {
            LoadConst => stack.push(self.load_const(instruction)? as u16),
            LoadBigConst => stack.push(self.load_big_const(instruction)?),
            IntAdd => stack.push(value),
            _ => todo!("Interpret bytecode")
        }
        Ok(())
    }
    /// Add the last 2 items on the stack
    fn int_add(stack: &mut Vec<u16>) -> u16 {
        let first = stack.pop().ol;

    }
    /// Get a u8
    fn load_const(&mut self, prev_byte: u8) -> Result<u8, InvalidBytecode> {
        self.next().ok_or_else(||InvalidBytecode::expected_byte(prev_byte))
    }
    /// Read in a u16 that is split across 2 u8s
    fn load_big_const(&mut self, prev_byte: u8) -> Result<u16, InvalidBytecode> {
        let first_byte = self.next().ok_or_else(||InvalidBytecode::expected_byte(prev_byte))?;
        let second_byte = self.next().ok_or_else(||InvalidBytecode::expected_byte(first_byte))?;
        let first_half: u16 = (first_byte as u16) << 8;
        Ok(first_half+(second_byte as u16))
    }

}
pub fn main(bytecode: &[u8]) -> Result<(), InvalidBytecode>{
    let bytecode = std::iter::from_fn(move || {

    })
}
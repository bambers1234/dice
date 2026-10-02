use std::sync::RwLock;

use super::bytecode::Bytecode::{self, *};
use super::errors::InvalidBytecode;

static STACK: RwLock<Vec<u8>> = RwLock::new(Vec::new());

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
    fn new(bytecode: &[u8]) -> Self {
        let vec = Vec::from(bytecode).into_iter();
        Self { bytes: vec }
    }
    fn next_instruction(&mut self, stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
        let next = self.next();
        if next.is_none() {
            todo!("End of bytecode")
        }
        #[allow(clippy::unwrap_used)]
        let instruction = next.unwrap();
        let code = Bytecode::try_from(instruction)?;
        match code {
            LoadConst => stack.push(self.load_const(instruction)?),
            IntAdd => Self::int_add(stack)?,
            _ => todo!("Interpret bytecode")
        }
        Ok(())
    }
    /// Add the last 2 items on the stack and push the result to the stack
    fn int_add(stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
        let first = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(IntAdd))?;
        let second = stack.pop().ok_or_else(|| InvalidBytecode::not_enough_on_stack(IntAdd))?;
        stack.push(first+second);
        Ok(())
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
pub fn main(bytecode: &[u8]) -> Result<(), InvalidBytecode> {
    let cursor = Cursor::new(bytecode);
    // execute and interpret simultaneously
    // execution in next() method
    // TODO: execution in next() is confusing architecture
    cursor.for_each(|byte| {} );
}
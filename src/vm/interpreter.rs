use std::sync::RwLock;

use super::bytecode::Bytecode;
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
}
pub fn execute(bytecode: &[u8]) -> Result<(), InvalidBytecode> {
    let mut bytes = Cursor::new(bytecode);
    let mut stack = STACK.write().expect("There has been a panic in the interpreter, causing the memory stack to poison.\nThis is a bug in the interpreter.");
    while let Some(byte) = bytes.next() { // while bc for loop causes cannot borrow mutably more than once at a time
        Bytecode::try_from(byte)?.execute(&mut stack, &mut bytes)?;
    }
    // display the final item on the stack, the result of the whole computation
    println!("{}", { if let Some(output) = stack.pop() {
        output.to_string()
    } else {
        "".to_string()
    }});
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::bytecode_defs::*;
    use super::*;

    #[test]
    fn execution() {
        execute(&[LOAD_CONST, 1, LOAD_CONST, 1, INT_ADD]).unwrap();
    }
}
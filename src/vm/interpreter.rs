use std::io::Write;
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
    
        exe_expr(&mut bytes, &mut *stack)?;
        // display the final item on the stack, the result of the whole computation
        print!("{}", { if let Some(output) = stack.pop() {
            format!("{}\n", output.to_string())
        } else {
            "".to_string()
        }});
        std::io::stdout().flush().unwrap();

    // each expr has its own stack frame (and without function calls, previous frames are inaccessible)
    stack.clear();
    Ok(())
}
fn exe_expr(bytes: &mut Cursor, stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
    while let Some(byte) = bytes.next() { // while bc for loop causes cannot borrow mutably more than once at a time
        if Bytecode::try_from(byte)?.execute(stack, bytes)?.is_break() {
            return Ok(())
        };
    }
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
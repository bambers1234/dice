use std::io::Write;
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

/// An iterator over discrete expressions
#[derive(Clone, Debug)]
struct Expressions<'a> {
    exprs: &'a [Bytes<'a>],
    index: usize,
}
impl<'a> Iterator for Expressions<'a> {
    type Item = Bytes<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.exprs.len() {
            return None
        }
        Some(self.exprs[self.index])
    }
}
impl<'a> From<Bytes<'a>> for Expressions<'a> {
    fn from(value: Bytes) -> Self {
        todo!("Find a way to split bytecode into expressions. A special bytecode sequence cannot be used as it could share a value with an integer.")
    }
}
/// Receive a byte stream which may or may not be valid bytecode and execute it. 
/// ## Errors: Invalid bytecode
pub fn execute(bytecode: Bytes) -> Result<(), InvalidBytecode> {
    let mut stack = STACK.write().expect("There has been a panic in the interpreter, causing the memory stack to poison.\nThis is a bug in the interpreter.");
    for mut expression in Expressions::from(bytecode) {
        exe_expr(&mut expression, &mut *stack)?;
        // display the final item on the stack, the result of the whole computation (print after each expression is executed)
        print!("{}", { if let Some(output) = stack.pop() {
            format!("{}\n", output.to_string())
        } else {
            "".to_string()
        }});
        std::io::stdout().flush().unwrap();
        // each expr has its own stack frame (and without function calls, previous frames are inaccessible)
        stack.clear();
    }
    Ok(())
}
/// Execute a single expression which may or may not be entirely valid bytecode.
/// ## Errors:
/// Invalid bytecode
fn exe_expr(bytes: &mut Bytes, stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
    while let Some(byte) = bytes.next() {
        let byte = Bytecode::try_from(byte)?;
        byte.execute(stack, bytes)?;
    }
    Ok(())
}

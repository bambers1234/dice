use std::io::Write;
use std::sync::RwLock;
use super::bytecode::Bytecode;
use super::errors::InvalidBytecode;

static STACK: RwLock<Vec<u8>> = RwLock::new(Vec::new());

#[derive(Debug, Clone)]
struct Bytes<'a> {
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
impl<'a> Bytes<'a> {
    fn new(bytecode: &'a [u8]) -> Self {
        Self { bytes: bytecode, index: 0 }
    }
}
struct Bytecodes {
    bytes: Vec<Bytecode>,
    index: usize,
}
impl TryFrom<Bytes<'_>> for Bytecodes {
    type Error = InvalidBytecode;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let mut bytecode = Vec::new();
        for byte in value {
            bytecode.push(Bytecode::try_from(byte)?);
        };
        Ok(Self {bytes: bytecode, index: 0})
    }
}
impl From<Vec<Bytecode>> for Bytecodes {
    fn from(value: Vec<Bytecode>) -> Self {
        Self {
            bytes: value,
            index: 0,
        }
    }
}
impl Iterator for Bytecodes {
    type Item = Bytecode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.bytes.len() {
            return None
        };
        self.index += 1;
        Some(self.bytes[self.index - 1])
    }
}
impl Bytecodes {
    fn expressions(&self) -> impl Iterator<Item = Bytecodes> {
        self.bytes.split(|x| x.expression_flow().is_end()).filter(|&x| !x.is_empty()).map(|x| Bytecodes::from(x.to_vec()))
    }
}
pub fn execute(bytecode: &[u8]) -> Result<(), InvalidBytecode> {
    let code = Bytecodes::try_from(Bytes::new(bytecode))?;
    let mut stack = STACK.write().expect("There has been a panic in the interpreter, causing the memory stack to poison.\nThis is a bug in the interpreter.");
    for expression in code.expressions() {
        exe_expr(&mut bytes, &mut *stack)?;
        // display the final item on the stack, the result of the whole computation
        print!("{}", { if let Some(output) = stack.pop() {
            format!("{}\n", output.to_string())
        } else {
            "".to_string()
        }});
        std::io::stdout().flush().unwrap();
    }
    while let Some(expr) = code.expressions().next() {
        exe_expr(&mut expr, &mut stack);
    }
    // each expr has its own stack frame (and without function calls, previous frames are inaccessible)
    stack.clear();
    Ok(())
}
fn exe_expr(bytes: &mut Bytecodes, stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
    for byte in bytes {
        if byte.execute(stack, bytes)?.is_end() {
            break;
        }
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
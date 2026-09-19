//! The final representation of the code. This module is for generating bytecode. For interpreting it, see the interpreter module.
#![allow(unused)]
use crate::ast::Node;

fn ast_to_bytecode<'a>(ast: Node) -> &'a [u8] {
    todo!("Convert ast to bytecode, design bytecode etc")
}

mod file {
    //! Writing bytecode to a file as raw bytes
    use crate::errors::InvalidBytecode;
    use std::fs::File;
    use std::io::Write;
    /// Write bytes to a file, returning an error and writing nothing if they are not valid bytecode.
    /// ## Performance
    /// Checking that every byte is a valid bytecode incurs performance cost. If you're certain that the bytecode is valid, use to_file_unchecked instead. <br>
    /// ## Errors
    /// If the I/O operation fails, or the bytecode is invalid
    fn to_file(bytecode: &[u8], file: File) -> Result<(), Box<dyn std::error::Error>>{
        todo!("Write bytecode to file and check its validity")
    }
    /// Write bytes to a file, assuming they are valid bytecode.
    /// ## Safety
    /// Only use this function if you are sure the bytes are valid bytecode, as invalid bytecode could cause undefined behaviour when interpreted.
    /// <br> For example, writing 0x00 to a file will be read as End of File by the interpreter. 
    /// ## Performance
    /// This function is much faster than its safe variant, to_file.
    /// ## Errors
    /// If the I/O operation fails
    unsafe fn to_file_unchecked(bytecode: &[u8], file: &mut File) -> std::io::Result<()> {
        file.write_all(bytecode)
    }
}
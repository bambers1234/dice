//! The final representation of the code. This module is for generating bytecode. For interpreting it, see the interpreter module.
#![allow(unused)]
use super::ast::Node;

pub fn ast_to_bytecode<'a>(ast: Node) -> &'a [u8] {
    todo!("Convert ast to bytecode, design bytecode etc")
}
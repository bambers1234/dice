//! The final representation of the code. This module is for generating bytecode. For interpreting it, see the interpreter module.
#![allow(unused)]
use super::ast::Node;

pub fn ast_to_bytecode<'a>(ast: Node) -> &'a [u8] {
    // What do I need the ast to return?
    // 1d6+1 = 
    //      Add
    //      / \
    //   Roll  1
    //   /  \
    //  1    6
    // in bytecode:
    // LOAD_CONST 1     
    // LOAD_CONST 6
    // DICE_ROLL
    // LOAD_CONST 1
    // ADD
    // order of traversal: l_param, r_param, self (post order traversal)
    todo!("Convert ast to bytecode, design bytecode etc")
}

use Node::*;
use crate::bytecode_defs::*;
/// Operations on the ast specific to generating bytecode
impl Node {
    pub fn evaluate(&self) -> &[u8] {
        match self {
            NumberLiteral { val } => 
        }
    }
}
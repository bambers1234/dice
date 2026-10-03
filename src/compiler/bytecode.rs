//! The final representation of the code. This module is for generating bytecode. For interpreting it, see the interpreter module.
#![allow(unused)]
use super::ast::Node;

pub fn ast_to_bytecode<'a>(ast: Node) -> Vec<u8> {
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
    ast.evaluate()
}

use Node::*;
use crate::bytecode_defs::*;
/// Operations on the ast specific to generating bytecode
impl Node {
    /// Evaluate the ast as bytecode using a post-order traversal
    pub fn evaluate(&self) -> Vec<u8> {
        match self {
            NumberLiteral { val } => vec![LOAD_CONST, *val],
            Op { l_param, op_type, r_param } => {
                l_param.evaluate().into_iter().chain(r_param.evaluate()).chain([op_type.as_bytecode()]).collect()
            }
        }
    }
}
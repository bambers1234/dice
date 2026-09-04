use crate::tokens::tokenisation::Token::{self, *};
use crate::errors::SyntaxError;
trait Node {
    fn walk(&self);
}
#[derive(Debug)]
struct DiceRoll {
    l_param: Number,
    r_param: Number,
}
#[derive(Debug)]
struct Number {
    value: u32,
}
#[derive(Debug)]
struct Ast<T: Node> {
    first_node: T,
}
impl<T: Node> Ast<T> {
    fn new(first_node: T) -> Ast<T> {
        Ast {
            first_node
        }
    }
}
pub fn generate_ast<T: Iterator<Item=Token>>(tokens: T) -> Result<Ast, SyntaxError>{
    let tree = 
    for token in tokens {
        match token {
            End => todo!(),
            DiceRollInvokation => todo!(),
            Token::Number {value} => todo!(),
            Unknown => return Err(SyntaxError::new(token)),
        }
    }
}


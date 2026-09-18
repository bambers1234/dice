//! Use a Pratt parser to produce an expression tree for the tokens.
use std::iter::Peekable;
use crate::tokens::tokenisation::Token::{self, *};
use Node::*;
use crate::operator::Operator::{self, *};

#[derive(Clone, Debug)]
enum Node {
    // leaf
    NumberLiteral{ val: u32 },
    // internal node has subtree either side
    Op{ l_param: Box<Node>, op_type: Operator, r_param: Box<Node> },
    // although RollInvokation is very similar to an operator, unlike ops, it cannot ever be compile-time evaluated
    RollInvokation{ amount: Box<Node>, sides: Box<Node>}
}
impl Node {
    /// Convert a given Operator into the corresponding expression tree node
    fn from_op(lexer: &mut Peekable<impl Iterator<Item = Token>>, op: Operator, l_param: Node) -> Node {
        Op { l_param: Box::new(l_param), op_type: op, r_param: Box::new(parse(lexer, op.precedence())) }
    }
}
/// Use a Pratt operator-precedence parser to generate an expression tree from a token supplier
fn parse(lexer: &mut Peekable<impl Iterator<Item = Token>>, precedence: u32) -> Node {
    let head_token = lexer.next().unwrap(); // None = EOF
    let mut left = null_denotation(lexer, head_token);
    loop {
        let next = lexer.peek().unwrap();
        if let Token::Operator { op_type: op } = next {
            if op.precedence() > precedence {
                let curr_token = lexer.next().unwrap(); // None = EOF
                left = left_denotation(lexer, left, curr_token);
            } else {
                break
            }
        } else {
            break
        }
    }
    left
}
// not defined as method on Token because it clearly belongs in parser, not tokenisation
// these basically convert Token to Node by adding parameters
/// Given a token at the start of an expression, return the corresponding Node to start the expression
fn null_denotation(lexer: &mut Peekable<impl Iterator<Item = Token>>,token: Token) -> Node {
    match token {
        Number { value } => NumberLiteral { val: value },
        _ => todo!("syntax error: no others can be heads of expressions")
    }
}
// it makes sense for each node to own its parameters
// lexer must be mut bc it's an iterator
/// Given a token from inside an expression, return the corresponding Node with parameters
fn left_denotation(lexer: &mut Peekable<impl Iterator<Item = Token>>, left: Node, token: Token) -> Node {
    match token {
        Token::Operator {op_type: op} => Node::from_op(lexer, op, left),
        _ => todo!("no others have tail handlers yet")
    }
}

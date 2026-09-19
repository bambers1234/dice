//! Produce and manipulate an abstract syntax tree
use std::iter::Peekable;
use crate::{errors::CannotInline, tokens::Token::{self, *}};
use Node::*;
use crate::operator::Operator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    // leaf
    NumberLiteral{ val: u32 },
    // internal node has subtree either side
    // includes dice rolls
    Op{ l_param: Box<Node>, op_type: Operator, r_param: Box<Node> },
}
impl Node {
    /// Convert a given Operator into the corresponding expression tree node
    fn from_op(lexer: &mut Peekable<impl Iterator<Item = Token>>, op: Operator, l_param: Node) -> Node {
        Op { l_param: Box::new(l_param), op_type: op, r_param: Box::new(parse(lexer, op.precedence())) }
    }
    /// Traverse and modify an expression tree in-place to inline constants.
    pub fn inline(&mut self) -> Result<(), CannotInline> {
        // returns () because it modifies in place
        // if both params can be inlined, self = literal
        match self {
            NumberLiteral { val: _ } => Ok(()),
            Op { l_param, op_type, r_param } => {
                let l_inlined: Result<(), CannotInline> = l_param.inline(); // modifies IN PLACE- this Op's params change
                let r_inlined: Result<(), CannotInline> = r_param.inline(); // if they can't be, ? returns Err
                // return here because otherwise not both params will attempt inlining, but if either is Err then this node must not be inlined
                l_inlined?;
                r_inlined?;
                // extract the inner u32
                // panic is safe, ? ensures both params are NumberLiteral by this point
                #[allow(clippy::panic)]
                let NumberLiteral { val: l_val } = **l_param else {
                    panic!("This should be unreachable, as inline()? ensures both params are NumberLiteral by this point");
                };
                #[allow(clippy::panic)]
                let NumberLiteral { val: r_val } = **r_param else {
                    panic!("This should be unreachable, as inline()? ensures both params are NumberLiteral by this point");
                };
                // self was Op, now condensed
                *self = NumberLiteral { val: op_type.calculate(l_val, r_val)? };
                Ok(())
            }
        }
    }
}
/// Produce an expression tree from a given token supplier, returning the root node. 
/// ## Panics
/// If there is a syntax error in the input
pub fn parse(lexer: &mut Peekable<impl Iterator<Item = Token>>, precedence: u32) -> Node {
    // use a Pratt operator-precedence parser
    let head_token = lexer.next().expect("SYNTAX ERROR: expected token, found end of input");
    let mut left = null_denotation(lexer, head_token);
    loop {
        // infix tokens MUST be operators in all scenarios for a Pratt Parser (Pratt parser can only do operators)
        if let Some(next) = lexer.peek() && let Token::Operator { op_type: op} = next && op.precedence() > precedence {
            #[allow(clippy::unwrap_used)]
            let curr_token = lexer.next().unwrap(); // unwrap is safe- checked above
            left = left_denotation(lexer, left, curr_token);
        } else {
            break; // None = EOF, go straight to returning Node
        }
    }
    left
}
// not defined as method on Token because it clearly belongs in parser, not tokenisation
// these basically convert Token to Node by adding parameters
/// Given a token at the start of an expression, return the corresponding Node to start the expression
fn null_denotation(_lexer: &mut Peekable<impl Iterator<Item = Token>>, token: Token) -> Node {
    match token {
        Number { value } => NumberLiteral { val: value },
        #[allow(clippy::panic)]
        _ => panic!("SYNTAX ERROR: {} cannot start an expression", token)
    }
}
// it makes sense for each node to own its parameters
// lexer must be mut bc it's an iterator
/// Given a token from inside an expression, return the corresponding Node with parameters
fn left_denotation(lexer: &mut Peekable<impl Iterator<Item = Token>>, left: Node, token: Token) -> Node {
    match token {
        Token::Operator {op_type: op} => Node::from_op(lexer, op, left),
        #[allow(clippy::panic)]
        _ => panic!("SYNTAX ERROR: {} cannot appear within an expression", token)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::tokenise;
    use crate::operator::Operator::*;
    use super::*;
    // relies on tokenising being correct
    #[test]
    fn add() {
        let mut tokens = tokenise("1+1");
        let root = parse(&mut tokens, 0);
        assert_eq!(root, Op { l_param: Box::new(NumberLiteral { val: 1 }),
            op_type: Add, r_param: Box::new(NumberLiteral { val: 1 })});
    }
    #[test] 
    fn mul() {
        let mut tokens = tokenise("1*1");
        let root = parse(&mut tokens, 0);
        assert_eq!(root, Op { l_param: Box::new(NumberLiteral { val: 1 }), 
            op_type: Mul, r_param: Box::new(NumberLiteral { val: 1 }) });
    }
    #[test]
    fn bidmas() {
        let mut tokens = tokenise("2+3*5");
        let root = parse(&mut tokens, 0);
        //      Add
        //      / \
        //     2  Mul
        //        / \
        //       3   5
        assert!(root == Op { 
            l_param: Box::new(NumberLiteral { val: 2 }), 
            op_type: Add, 
            r_param: Box::new(Op { 
                l_param: Box::new(NumberLiteral { val: 3 }), 
                op_type: Mul, 
                r_param: Box::new(NumberLiteral { val: 5 }) 
            }) 
        })
    }
    #[test]
    fn inline() {
        let mut tokens = tokenise("2+3*4");
        //      Add
        //      / \
        //     2  Mul
        //        / \
        //       3   4
        let mut root = parse(&mut tokens, 0);
        root.inline().unwrap();
        assert!(root == NumberLiteral { val: 14 })
    }
    #[test]
    fn dice_op() {
        let mut lexer = tokenise("1d6+1");
        //      Add
        //      / \
        //    Dice 1
        //    /  \
        //   1    6
        let root = parse(&mut lexer, 0);
        assert_eq!(root, Op {
            l_param: Box::new(Op { 
                l_param: Box::new(NumberLiteral { val: 1 }), 
                op_type: Dice, 
                r_param: Box::new(NumberLiteral { val: 6 })
            }), 
            op_type: Add, 
            r_param: Box::new(NumberLiteral { val: 1 }) 
        });
    }
    #[test]
    fn partial_inline() {
        let mut lexer = tokenise("1d6+1*2");
        //      Add
        //     /   \
        //  Dice    Mul
        //  /  \    / \
        // 1    6  1   2
        let mut root = parse(&mut lexer, 0);
        assert_eq!(root, Op {
            l_param: Box::new(Op { 
                l_param: Box::new(NumberLiteral { val: 1 }), 
                op_type: Dice, 
                r_param: Box::new(NumberLiteral { val: 6 }),
            }),
            op_type: Add,
            r_param: Box::new(Op {
                l_param: Box::new(NumberLiteral { val: 1 }),
                op_type: Mul,
                r_param: Box::new(NumberLiteral { val: 2 }),
            })
        });
        assert!(root.inline().is_err());
        //      Add
        //      / \
        //   Dice  2
        //   /  \
        //  1    6
        // none of the tree is being inlined- it isn't changing
        assert_eq!(root, Op {
            l_param: Box::new(Op {
                l_param: Box::new(NumberLiteral { val: 1 }),
                op_type: Dice,
                r_param: Box::new(NumberLiteral { val: 6 }),
            }),
            op_type: Add,
            r_param: Box::new(NumberLiteral { val: 2 }),
        })
    }
}
use crate::bytecode_defs::NEW_EXPRESSION;

mod errors;
mod tokens;
mod ast;
mod bytecode;
mod operator;

pub fn compile(input: &'static str) -> Vec<u8> {
    let mut tokens = tokens::tokenise(input);
    // need to split tokens into exprs
    // then generate ast for each
    // and convert each to bytecode
    let mut ast = ast::parse(&mut tokens, 0);
    let _ = ast.inline(); // let _ bc it doesn't matter if not everything can be inlined
    let mut evaluated = ast.evaluate();
    evaluated.push(NEW_EXPRESSION);
    evaluated
}

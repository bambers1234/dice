mod errors;
mod tokens;
mod ast;
mod bytecode;
mod operator;

pub fn compile<'a>(input: &'static str) -> &'a [u8]{
    let mut tokens = tokens::tokenise(input);
    let mut ast = ast::parse(&mut tokens, 0);
    let _ = ast.inline(); // let _ bc it doesn't matter if not everything can be inlined
    todo!("Convert ast to bytecode");
}

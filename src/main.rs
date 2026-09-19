#![warn(clippy::unwrap_used)]

use std::sync::OnceLock;

mod dice;
mod errors;
mod tokens;
mod ast;
mod cli;
mod bytecode;
mod operator;

static INPUT: OnceLock<String> = OnceLock::new();
fn main() {
    loop {
        // keep retrying until stdin works
        // risk of infinite loop?
        INPUT.get_or_init(|| input());
        compile(INPUT.get().unwrap());
    }
}
fn compile(input: &'static str) {
    let mut tokens = tokens::tokenise(input);
    let ast = ast::parse(&mut tokens, 0);
}
fn input() -> String {
    let mut buf = String::new();
    while std::io::stdin().read_line(&mut buf).is_err() {};
    buf
}
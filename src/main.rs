#![warn(clippy::unwrap_used)]
#![warn(clippy::panic)]
use std::sync::OnceLock;

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
        INPUT.get_or_init(|| sanitise(input()));
        #[allow(clippy::unwrap_used)]
        // unwrap is safe- INPUT initialised above
        compile(INPUT.get().unwrap());
    }
}
fn compile(input: &'static str) {
    let mut tokens = tokens::tokenise(input);
    let mut ast = ast::parse(&mut tokens, 0);
    let _ = ast.inline(); // let _ bc it doesn't matter if not everything can be inlined
}
/// Transform an input &str to remove leading and trailing whitespace and switch to lowercase
fn sanitise<T: ToString>(input: T) -> String {
    input.to_string().trim().to_lowercase()
}
/// Get a single line of stdin input
fn input() -> String {
    let mut buf = String::new();
    while std::io::stdin().read_line(&mut buf).is_err() {};
    buf
}
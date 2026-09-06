#![warn(clippy::unwrap_used)]

use crate::tokens::tokenisation::tokenise;
mod dice;
mod errors;
mod tokens;
mod ast;
mod command;
mod cli;
mod chars;
mod bytecode;
fn main() {
    loop {
        let mut input = String::new();
        // keep retrying until stdin works
        // risk of infinite loop?
        while std::io::stdin().read_line(&mut input).is_err() {};
        let tokens = tokens::parse(&input);
    }
}
pub fn compile(input: String) {
    let tokens = tokenise(&input);

}

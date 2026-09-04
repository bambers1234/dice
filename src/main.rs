#![warn(clippy::unwrap_used)]
mod dice;
mod errors;
mod tokens;
mod ast;
fn main() {
    loop {
        let mut input = String::new();
        // keep retrying until stdin works
        // risk of infinite loop?
        while std::io::stdin().read_line(&mut input).is_err() {};
        let tokens = tokens::parse(&input);
    }
}

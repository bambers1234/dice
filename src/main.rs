#![warn(clippy::unwrap_used)]
#![warn(clippy::panic)]
use std::sync::OnceLock;
use crate::compiler::compile;
use crate::cli::Cli;

mod compiler;
mod vm;
mod cli;

static INPUT: OnceLock<String> = OnceLock::new();
fn main() {
    let cli = Cli::parse();
    let input = INPUT.get_or_init(||cli.input());
    compile(input);
}

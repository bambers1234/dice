#![warn(clippy::unwrap_used)]
#![warn(clippy::panic)]
use std::sync::OnceLock;
use crate::compiler::compile;
use crate::cli::Cli;

mod compiler;
mod vm;
mod cli;
mod bytecode_defs;

static INPUT: OnceLock<String> = OnceLock::new();
fn main() {
    let cli = Cli::parse();
    let input = INPUT.get_or_init(||cli.input());
    let bytecode = compile(input);
    if cli.run_immediately() {
        crate::vm::execute(&bytecode);
    }
    if let Some(path) = cli.write_out() {
        write_to_file(&bytecode, path);
    }
}
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
fn write_to_file(bytecode: &[u8], file: &Path) {
    #[allow(clippy::panic)] // desired
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file).unwrap_or_else(|error| panic!("Error opening supplied output file:\n{}", error)); 
    let mut file = BufWriter::new(file);
    #[allow(clippy::panic)] // desired
    file.write_all(bytecode).unwrap_or_else(|error| panic!("Error writing to supplied output file, even though it opened successfully:\n{}", error));
}

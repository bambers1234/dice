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

    }
}
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
fn write_to_file(bytecode: &[u8], file: &Path) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file).unwrap();
    let mut file = BufWriter::new(file);
    file.write_all(bytecode).unwrap();
}

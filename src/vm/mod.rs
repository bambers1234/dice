mod bytecode;
mod interpreter;
mod dice;
mod errors;

pub fn execute(bytecode: &[u8]) {
    interpreter::execute(bytecode);
}
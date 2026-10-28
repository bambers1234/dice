mod bytecode;
mod interpreter;
mod dice;
mod errors;

pub fn execute(bytecode: &[u8]) {
    if let Err(err) = interpreter::execute(interpreter::Bytes::from(bytecode)) {
        println!("{}\nThis is an error with the generated compiled code. If you have not writted the binary directly, this is an error with the compiler.", err)
    };
}
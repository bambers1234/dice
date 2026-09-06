//! When the user's code is compiled, it will be compiled to bytecode. The different bytecodes are defined here.
use strum::EnumString;
use std::fs::File;
/// The bytecode operations
#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum Bytecode {
    #[strum(serialize="ROLL")]
    Roll,
    #[strum(serialize="NUM")]
    Number
}
impl Bytecode {
    fn read(file: File) {
        todo!();
    }
}
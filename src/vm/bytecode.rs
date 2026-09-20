#[allow(unused)]
use Bytecode::*;

use crate::vm::errors::InvalidBytecode;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Bytecode {
    LoadConst,
    LoadBigConst,
    IntAdd,
    IntMul,
    DiceRoll,
}
impl TryFrom<u8> for Bytecode {
    type Error = InvalidBytecode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LoadConst),
            1 => Ok(LoadBigConst),
            2 => Ok(DiceRoll),
            3 => Ok(IntAdd),
            4 => Ok(IntMul),
            _ => Err(InvalidBytecode::unknown_byte(value))
        }
    }
}
// TODO: use a macro to auto-generate TryFrom<u8> for Bytecode due to large number of variants
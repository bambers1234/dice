use strum::{EnumString};
#[allow(unused)]
use Bytecode::*;

use crate::vm::errors::InvalidBytecode;
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString)]
#[repr(u8)]
pub enum Bytecode {
    LoadConst,
    IntAdd,
    IntMul,
    DiceRoll,
}
impl TryFrom<u8> for Bytecode {
    type Error = InvalidBytecode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LoadConst),
            1 => Ok(DiceRoll),
            2 => Ok(IntAdd),
            3 => Ok(IntMul),
            _ => Err(InvalidBytecode::unknown_byte(value))
        }
    }
}
impl std::fmt::Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// TODO: use a macro to auto-generate TryFrom<u8> for Bytecode due to large number of variants
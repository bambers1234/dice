//! The bytes that represent bytecode instructions are defined here to ensure consistency between the vm and compiler
pub const INT_ADD: u8 = 0x01;
pub const INT_MUL: u8 = 0x02;
pub const LOAD_CONST: u8 = 0x03;
pub const DICE_ROLL: u8 = 0x04;
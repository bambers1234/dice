#[allow(unused)]
use Bytecode::*;

use crate::vm::errors::InvalidBytecode;
use crate::bytecode_defs::*;
use execution::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Bytecode { // safer than raw bytes- holds the invariant of being known instructions
    LoadConst,
    IntAdd,
    IntMul,
    DiceRoll,
}
impl TryFrom<u8> for Bytecode {
    type Error = InvalidBytecode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            LOAD_CONST => Ok(LoadConst),
            DICE_ROLL => Ok(DiceRoll),
            INT_ADD => Ok(IntAdd),
            INT_MUL => Ok(IntMul),
            _ => Err(InvalidBytecode::unknown_byte(value))
        }
    }
}
impl std::fmt::Display for Bytecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Bytecode {
    pub fn execute(&self, stack: &mut Vec<u8>, bytes: &mut impl Iterator<Item = u8>) -> Result<(), InvalidBytecode>{
        match self {
            LoadConst => load_const(bytes, stack),
            IntAdd => int_add(stack),
            IntMul => int_mul(stack),
            DiceRoll => roll_dice(stack),
            _ => todo!()
        }
    }   
}
mod execution {
    use crate::vm::errors::InvalidBytecode;
    use super::Bytecode::*;
    pub fn load_const(bytes: &mut impl Iterator<Item = u8>, stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
        stack.push(bytes.next().ok_or_else(|| InvalidBytecode::expected_byte())?);
        Ok(())
    }
    /// Add the last 2 items on the stack and push the result to the stack
    pub fn int_add(stack: &mut Vec<u8>) -> Result<(), InvalidBytecode>{
        let first = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(IntAdd))?;
        let second = stack.pop().ok_or_else(|| InvalidBytecode::not_enough_on_stack(IntAdd))?;
        stack.push(first+second);
        Ok(())
    }
    pub fn int_mul(stack: &mut Vec<u8>) -> Result<(), InvalidBytecode> {
        let first = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(IntMul))?;
        let second  = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(IntMul))?;
        stack.push(first*second);
        Ok(())
    }
    pub fn roll_dice(stack: &mut Vec<u8>) -> Result<(), InvalidBytecode> {
        let sides = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(DiceRoll))?;
        let amount = stack.pop().ok_or_else(||InvalidBytecode::not_enough_on_stack(DiceRoll))?;
        let dice = super::super::dice::roll(amount, sides);
        stack.push(dice.sum());
        Ok(())
    }
}
// TODO: use a macro to auto-generate TryFrom<u8> for Bytecode due to large number of variants
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_ops() {
        let mut stack: Vec<u8> = vec![1, 1];
        int_add(&mut stack).unwrap();
        assert_eq!(stack, vec![2]);
        stack.push(3);
        int_mul(&mut stack).unwrap();
        assert_eq!(stack, vec![6]);
    }
    #[test]
    fn load() {
        let mut bytes = vec![1].into_iter();
        let mut stack = Vec::new();
        load_const(&mut bytes, &mut stack).unwrap();
        assert_eq!(stack, vec![1]);
    }
    #[test]
    #[should_panic]
    fn not_enough_on_stack_for_int_op() {
        let mut stack = vec![1];
        int_add(&mut stack).unwrap();
    }
}
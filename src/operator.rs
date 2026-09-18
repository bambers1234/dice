use crate::errors::CannotInline;
use Operator::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
    Dice,
}
impl Operator {
    pub fn calculate(&self, l_param: u32, r_param: u32) -> Result<u32, CannotInline> {
        match self {
            Self::Add => Ok(l_param + r_param),
            Self::Mul => Ok(l_param * r_param),
            Self::Dice => Err(CannotInline {})
        }
    }
    pub fn precedence(&self) -> u32 {
        match self {
            Add => 1,
            Mul => 2,
            Dice => 3,
        }
    }
}
impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Add => String::from("plus (+)"),
            Mul => String::from("multiply (*)"),
            Dice => String::from("dice roll (d)"),
        }
    }
}
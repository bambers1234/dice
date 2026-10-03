use crate::compiler::errors::CannotInline;
use crate::bytecode_defs::*;
use Operator::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
    Dice,
    Div,
    Sub,
}
impl Operator {
    pub fn calculate(&self, l_param: u8, r_param: u8) -> Result<u8, CannotInline> {
        match self {
            Self::Add => Ok(l_param + r_param),
            Self::Mul => Ok(l_param * r_param),
            Self::Dice => Err(CannotInline {}),
            Self::Div => Ok(l_param/r_param),
            Self::Sub => Ok(l_param - r_param),
        }
    }
    pub fn precedence(&self) -> u8 {
        // higher precedence = becomes a parameter
        match self {
            Add => 1,
            Mul => 2,
            Dice => 3,
            Sub => 1,
            Div => 2
        }
    }
    pub fn as_bytecode(&self) -> u8 {
        match self {
            Add => INT_ADD,
            Mul => INT_MUL,
            Dice => DICE_ROLL,
            Div => INT_DIV,
            Sub => INT_SUB,
        }
    }
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => write!(f, "plus (+)"),
            Mul => write!(f, "multiply (*)"),
            Dice => write!(f, "dice roll (d)"),
            Div => write!(f, "divide (/)"),
            Sub => write!(f, "subtract (-)"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Operator::*;
    #[test]
    fn calc_add() {
        assert_eq!(Add.calculate(1, 2).unwrap(), 3);
    }
    #[test]
    fn calc_mul() {
        assert_eq!(Mul.calculate(1, 2).unwrap(), 2);
    }
    #[test]
    #[should_panic]
    fn calc_dice() {
        Dice.calculate(1, 6).unwrap();
    }
    #[test]
    fn strings() {
        assert_eq!(Add.to_string(), "plus (+)");
        assert_eq!(Mul.to_string(), "multiply (*)");
        assert_eq!(Dice.to_string(), "dice roll (d)");
    }
}
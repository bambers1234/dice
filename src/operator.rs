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
            Add => 2,
            Mul => 3,
            Dice => 1,
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
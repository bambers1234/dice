#![allow(unused)]
use crate::errors::SyntaxError;
type Amount = u32;
type Sides = u32;
/// Given an input string, extract the data about the dice roll from it.
/// ## Errors
/// If the provided string is not valid dice notation of the form XdN where X and N are positive numbers.
fn parse_dice_notation(notation: &str) -> Result<Roll, SyntaxError> {
    let mut parts = notation.splitn(2, 'd');
    let amount: Amount = parts
        .next()
        .ok_or_else(|| SyntaxError::new(notation))?
        .parse()
        .map_err(|_| SyntaxError::new(notation))?;
    let sides: Sides = parts
        .next()
        .ok_or_else(|| SyntaxError::new(notation))?
        .parse()
        .map_err(|_| SyntaxError::new(notation))?;
    Ok(Roll::new(amount, sides))
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Roll {
    number_to_roll: Amount,
    number_of_sides: Sides,
}
impl Roll {
    fn new(amount: Amount, sides: Sides) -> Roll {
        Roll {
            number_of_sides: sides,
            number_to_roll: amount,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dice() {
        let notation = "1d6";
        let roll = parse_dice_notation(notation).unwrap();
        assert_eq!(roll, Roll::new(1, 6));
    }
    #[test]
    #[should_panic]
    fn parse_dice_invalid() {
        let notation = "16";
        let _roll = parse_dice_notation(notation).unwrap();
    }
    #[test]
    #[should_panic]
    fn parse_dice_nonsense() {
        let notation = "hello";
        let _roll = parse_dice_notation(notation).unwrap();
    }
}

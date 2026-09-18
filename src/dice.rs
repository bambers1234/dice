#![allow(unused)]
use crate::errors::SyntaxError;
use rand::random_range;
type Amount = u32;
type Sides = u32;
/// Given an input string, extract the data about the dice roll from it.
/// ## Errors
/// If the provided string is not valid dice notation of the form XdN where X and N are positive numbers.
fn parse_dice_notation(notation: &str) -> Result<Roll, SyntaxError> {
    let mut parts = notation.splitn(2, 'd');
    let amount: Amount = parts
        .next()
        .ok_or_else(|| SyntaxError::new_tokenising_error(notation))?
        .parse()
        .map_err(|_| SyntaxError::new_tokenising_error(notation))?;
    let sides: Sides = parts
        .next()
        .ok_or_else(|| SyntaxError::new_tokenising_error(notation))?
        .parse()
        .map_err(|_| SyntaxError::new_tokenising_error(notation))?;
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
    fn roll(&self) -> u32 {
        let mut res = 0;
        for i in 0..self.number_to_roll {
            res += random_range(1..=self.number_of_sides)
        }
        res
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
    #[test]
    fn roll() {
        let dice = parse_dice_notation("1d6").unwrap();
        assert!((1..=6).contains(&dice.roll()));
    }
}

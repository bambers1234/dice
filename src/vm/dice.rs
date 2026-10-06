//! A compiler built-in way to roll dice
use RollType::*;
/// The output of a roll
#[derive(Debug, Clone)]
pub struct DiceRoll {
    rolls: Vec<RollType>
}
impl DiceRoll {
    pub fn sum(&self) -> u8 {
        self.rolls.iter().fold(0, |acc, &ele| acc + ele.into_value())
    }
    fn new(rolls: Vec<RollType>) -> Self {
        Self { rolls }
    }
    pub fn keep_highest(&mut self, num_to_keep: u8) {
        use itertools::Itertools;
        self.rolls = self.rolls.iter().sorted_by(|&x, &y| y.into_value().cmp(&x.into_value()))
            .map(|x| *x ).take(num_to_keep as usize).collect();
    }
    pub fn drop_lowest(&mut self, num_to_drop: u8) {
        
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RollType {
    Normal(u8),
    One,
    /// The maximum roll for a die of X sides
    Max(u8),
}
impl RollType {
    fn into_value(self) -> u8 {
        match self {
            Normal(num) => num,
            One => 1,
            Max(num) => num
        }
    }
}

pub fn roll(amount: u8, sides: u8) -> DiceRoll {
    let mut rolls = Vec::new();
    for _i in 0..amount {
        let res = rand::random_range(1..=sides);
        if res == sides {
            rolls.push(RollType::Max(res));
        }
        else if res == 1 {
            rolls.push(RollType::One)
        }
        else {
            rolls.push(RollType::Normal(res))
        }
    }
    DiceRoll::new(rolls)
}
#[cfg(test)]
mod tests {
    use crate::vm::dice::RollType;
    use super::*;
    #[test]
    fn keep_highest() {
        let mut dice = DiceRoll{ rolls: vec![RollType::One, RollType::Normal(2), RollType::Normal(3), RollType::Normal(4), RollType::Normal(5), RollType::Max(6)] };
        dice.keep_highest(2);
        assert_eq!(dice.rolls, vec![RollType::Max(6), RollType::Normal(5)]);
    }
}
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
}
#[derive(Clone, Copy, Debug)]
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
    for i in 0..amount {
        let res = rand::random_range(1..=sides);
        if res == sides {
            rolls.push(RollType::Max(res));
        }
        if res == 1 {
            rolls.push(RollType::One)
        }
        else {
            rolls.push(RollType::Normal(res))
        }
    }
    DiceRoll::new(rolls)
}
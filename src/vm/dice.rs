//! A compiler built-in way to roll dice
use RollType::*;
/// The output of a roll
#[derive(Debug, Clone)]
struct DiceRoll {
    rolls: Vec<RollType>
}
impl DiceRoll {
    fn sum(&self) -> u32 {
        self.rolls.iter().fold(0, |acc, &ele| acc + ele.into_value())
    }
    fn new(rolls: Vec<RollType>) -> Self {
        Self { rolls }
    }
}
#[derive(Clone, Copy, Debug)]
enum RollType {
    Normal(u32),
    One,
    /// The maximum roll for a die of X sides
    Max(u32),
}
impl RollType {
    fn into_value(self) -> u32 {
        match self {
            Normal(num) => num,
            One => 1,
            Max(num) => num
        }
    }
}

fn roll(amount: u32, sides: u32) -> DiceRoll {
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
use std::convert::Infallible;
use std::error::Error;
use crate::command::Command::{self, *};
use crate::tokens::tokenisation::Token::{self, *};
trait Node {
    type Meaning;
    type Error: Error;
    fn to_command(&self) -> Result<Self::Meaning, Self::Error>;
}
#[derive(Debug)]
struct DiceRoll {
    l_param: Number,
    r_param: Number,
}
impl Node for DiceRoll {
    type Meaning = Command;
    type Error = Infallible;
    fn to_command(&self) -> Result<Self::Meaning, Self::Error>{
        Ok(Roll { amount: self.l_param.to_command()?, sides: self.r_param.to_command()? })
    }
}
#[derive(Debug)]
struct Number {
    value: u32,
}
impl Node for Number {
    type Meaning = u32;
    type Error = Infallible;
    fn to_command(&self) -> Result<Self::Meaning, Self::Error>{
        Ok(self.value)
    }
}
#[derive(Debug)]
struct Ast<T: Node> {
    first_node: T,
}
impl<T: Node> Ast<T> {
    fn new(first_node: T) -> Ast<T> {
        Ast {
            first_node
        }
    }
}



use std::{iter::Peekable, process::exit};
use Token::*;
use super::errors::SyntaxError;
use crate::compiler::{operator::{Operator::{self, *}, DiceOps::{self, *}}};
use itertools::Itertools;
use std::str::Chars;
/// Turn a given input string into a sequence of commands. Case sensitive and sensitive to leading and trailing whitespace
pub fn tokenise(input: &'static str) -> Peekable<impl Iterator<Item = Token>> {
    // vec is annoying to need, but iterators are lazy
    // so the tokenised string is used after the function returns
    // And does not live long enough
    // Unless we consume the iterator to produce a Vec, forcing it to completely drain before returning
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if let End = token { None } else {Some(token)}
    }).peekable()
}
#[derive(Debug)]
pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
} 
impl<'a> Iterator for Cursor<'a> {
    // needs to be peeking- how to impl PeekingNext
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}
impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        // peek() consumes the first and only the first element
        Cursor {
            chars: input.chars().peekable(),
        }
    }
    fn advance_token(&mut self) -> Token {
        // skip whitespace before a token, removing it from the input stream via for_each
        self.chars.peeking_take_while(|&x| x.is_whitespace()).for_each(|_x|{});
        let Some(char) = self.chars.peek() else {
            return End;
        };
        // remember that every single function to eat chars must consume the chars it turns into tokens, 
        // otherwise a character will be repeatead
        // the check that it's not None is done above, unwrap is safe
        #[allow(clippy::unwrap_used)]
        #[allow(clippy::panic)]
        let token = match char.to_lowercase().next().unwrap() {
            // numeric literal
            '0'..='9' => Number {
                value: self.eat_digits(),
            },
            
            'd' => {self.starts_d().unwrap_or_else(|err| panic!("{}", err)) },
            ';' => {self.next(); End},
            '+' => {self.next(); Token::Operator { op_type: Add }},
            '*' => {self.next(); Token::Operator { op_type: Mul }},
            '/' => {self.next(); Token::Operator { op_type: Div }},
            '-' => {self.next(); Token::Operator { op_type: Sub }},
            'k' => {self.next(); Token::Operator { op_type: Operator::Dice(KeepHighest) }},
            _ => panic!("{}", SyntaxError::unknown_token(Unknown { pat: self.unknown() })),
        };
        token
    }
    /// Continue to consume chars whilst in a digit, then returning the numeric value of those digits.
    fn eat_digits(&mut self) -> u8 {
        let mut number = String::new();
        self.chars.peeking_take_while(|ch|ch.is_numeric()).for_each(|ch|number.push(ch));
        // unwrap is safe- the loop breaks as soon as a non-digit char is encountered,
        // so all chars are certain to be digits.
        // BUT possible overflow = panic
        number.parse::<u8>().unwrap()
    }
    fn starts_d(&mut self) -> Result<Token, SyntaxError> {
        #[allow(clippy::unwrap_used)]
        self.next().unwrap();
        if let Some(char) = self.chars.peek() {
            if *char == 'l' {
                self.next().ok_or_else(|| SyntaxError::token_not_found(String::from("d")))?;
                Ok(Token::Operator { op_type: Operator::Dice(DiceOps::DropLowest) })
            } else if char.is_numeric() {
                Ok(Token::Operator { op_type: Operator::Dice(DiceOps::Dice) })
            } else {
                let mut pat = String::from("d");
                pat.push_str(&self.unknown());
                Err(SyntaxError::unknown_token(Unknown { pat }))
            }
        } else {
           Err(SyntaxError::token_not_found(String::from("d")))
        }
    }
    fn unknown(&mut self) -> String {
        let mut pat = String::new();
        self.chars.peeking_take_while(|ch|!ch.is_whitespace() && *ch != ';')
            .for_each(|ch|pat.push(ch));
        pat
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// The possible operable token types.
pub enum Token {
    End,
    // only positive ints are supported
    Number { value: u8 },
    Operator {op_type: Operator },
    Unknown {pat: String},
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            End => write!(f, "End of Input"),
            Number { value } => write!(f, "Numeric literal '{}'", value),
            Unknown { pat } => write!(f, "'{}'", pat),
            Token::Operator { op_type } => write!(f, "Operator: {}", op_type),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_digits() {
        let mut cursor = Cursor::new("123");
        assert_eq!(cursor.eat_digits(), 123);
    }
    #[test]
    fn get_dice_tokens() {
        let mut tokens = tokenise("1d6");
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert_eq!(tokens.next().unwrap(), Token::Operator { op_type: Operator::Dice(DiceOps::Dice) });
        assert_eq!(tokens.next().unwrap(), Number { value: 6 });
        assert!(tokens.next().is_none());
    }
    #[test]
    #[should_panic]
    fn unknown_token() {
        let tokens = tokenise("blah\n");
        // consume because iterators are lazy
        tokens.for_each(|_| {});
    }
    #[test]
    fn add() {
        let mut tokens = tokenise("1+1;");
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert_eq!(tokens.next().unwrap(), Token::Operator { op_type: Add });
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert!(tokens.next().is_none());
    }
    #[test]
    fn mul() {
        let mut tokens = tokenise("1*1;");
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert_eq!(tokens.next().unwrap(), Token::Operator { op_type: Mul });
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert!(tokens.next().is_none())
    }
    #[test]
    fn sub() {
        let mut tokens = tokenise("1-1");
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert_eq!(tokens.next().unwrap(), Token::Operator { op_type: Sub });
        assert_eq!(tokens.next().unwrap(), Number {value: 1});
        assert!(tokens.next().is_none());
    }
    #[test]
    fn div() {
        let mut tokens = tokenise("1/1");
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert_eq!(tokens.next().unwrap(), Token::Operator { op_type: Div });
        assert_eq!(tokens.next().unwrap(), Number { value: 1 });
        assert!(tokens.next().is_none());
    }
    #[test]
    fn end() {
        let mut tokens = tokenise(";");
        assert!(tokens.next().is_none())
    }
}


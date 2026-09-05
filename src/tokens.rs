/// Turn a given input string into a sequence of commands. Not case sensitive.
pub fn parse(input: &str) -> Vec<tokenisation::Token>{
    // vec is annoying to need, but iterators are lazy
    // so the tokenised string is used after the function returns
    // And does not live long enough
    // Unless we consume the iterator to produce a Vec, forcing it to completely drain before returning
    let trimmed = String::from(input.to_lowercase().trim());
    tokenisation::tokenise(&trimmed).collect()
}
pub mod tokenisation {
    use Token::*;
    use itertools::Itertools;
    use regular_expressions::*;
    use std::{iter::Peekable, str::Chars};
    mod regular_expressions {
        //! The patterns to match against to determine the type of token
        // centralised here to be easily changeable if i want the syntax to change
        pub const ROLL_INVOKATION: char = 'd';
        pub const END_OF_INPUT: char = '\n';
    }
    pub fn tokenise(input: &str) -> impl Iterator<Item = Token> {
        let mut cursor = Cursor::new(input);
        std::iter::from_fn(move || {
            let token = cursor.advance_token();
            if let End = token { None } else { Some(token) }
        })
    }
    #[derive(Debug)]
    struct Cursor<'a> {
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
        fn new(input: &'a str) -> Cursor<'a> {
            // peek() consumes the first and only the first element
            Cursor {
                chars: input.chars().peekable(),
            }
        }
        fn advance_token(&mut self) -> Token {
            // no more characters = finished
            let Some(char) = self.chars.peek() else {
                return End;
            };
            // PROBLEM: ASCII only
            // remember that every single function to eat chars must actually eat the chars it turns into tokens, 
            // whether through peeking_take_while() or next()
            // otherwise a character will be repeatead
            #[allow(clippy::unwrap_used)]
            // the check that it's not None is done above, unwrap is safe
            let token = match char.to_lowercase().next().unwrap() {
                // numeric literal
                '0'..='9' => Number {
                    value: self.eat_digits(),
                },
                'd' => {self.next(); DiceRollInvokation},
                '\n' => End,
                // symbol not in regular language, hence invalid
                _ => Unknown {pat: self.unknown()},
            };
            token
        }
        /// Continue to consume chars whilst in a digit, then returning the numeric value of those digits.
        fn eat_digits(&mut self) -> u32 {
            let mut number = String::new();
            self.chars.peeking_take_while(|ch|ch.is_numeric()).for_each(|ch|number.push(ch));
            // unwrap is safe- the loop breaks as soon as a non-digit char is encountered,
            // so all chars are certain to be digits.
            #[allow(clippy::unwrap_used)]
            number.parse::<u32>().unwrap()
        }
        fn unknown(&mut self) -> String {
            let mut pat = String::new();
            self.chars.peeking_take_while(|ch|!ch.is_whitespace() && !(*ch == END_OF_INPUT))
                .for_each(|ch|pat.push(ch));
            pat
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    /// The possible operable token types.
    pub enum Token {
        End,
        // 'd'
        DiceRollInvokation,
        // only positive ints are supported
        Number { value: u32 },
        Unknown {pat: String},
    }
    impl ToString for Token {
        fn to_string(&self) -> String {
            match self {
                End => String::from("End of input"),
                DiceRollInvokation => format!("Dice roll invokation: '{}", ROLL_INVOKATION),
                Number { value } => format!("Numeric literal '{}'", value),
                Unknown{pat} => format!("Unknown token '{}'", pat),
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
            assert_eq!(tokens.next().unwrap(), DiceRollInvokation);
            assert_eq!(tokens.next().unwrap(), Number { value: 6 });
            assert!(tokens.next().is_none());
        }
        #[test]
        fn unknown_token() {
            let mut tokens = tokenise("blah\n");
            assert_eq!(tokens.next().unwrap(), Unknown { pat: String::from("blah")});
            assert!(tokens.next().is_none());
        }
    }
}

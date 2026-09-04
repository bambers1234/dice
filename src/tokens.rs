/// Turn a given input string into a sequence of commands. Not case sensitive.
pub fn parse(input: &str) -> Vec<tokenisation::Token>{
    // vec is annoying to need, but iterators are lazy 
    // so the tokenised string is used after the function returns
    // And does not live long enough
    // Unless we consume the iterator to produce a Vec, forcing it to completely drain before returning
    tokenisation::tokenise(input.to_lowercase().trim()).collect()
}
pub mod tokenisation {
    use Token::*;
    use regular_expressions::*;
    use std::str::Chars;
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
    struct Cursor<'a> {
        chars: Chars<'a>,
    }
    impl<'a> Iterator for Cursor<'a> {
        type Item = char;
        fn next(&mut self) -> Option<Self::Item> {
            self.chars.next()
        }
    }
    impl<'a> Cursor<'a> {
        fn new(input: &'a str) -> Cursor<'a> {
            Cursor {
                chars: input.chars(),
            }
        }
        fn advance_token(&mut self) -> Token {
            let Some(char) = self.next() else {
                return End;
            };
            // PROBLEM: ASCII only
            match char.to_ascii_lowercase() {
                // numeric literal
                c @ '0'..='9' => Number {
                    value: self.eat_digits(c),
                },
                ROLL_INVOKATION => DiceRollInvokation,
                END_OF_INPUT => End,
                // symbol not in regular language, hence invalid
                _ => Unknown,
            }
        }
        /// Continue to consume chars whilst in a digit, then returning the numeric value of those digits.
        fn eat_digits(&mut self, first_digit: char) -> u32 {
            let mut number = String::new();
            number.push(first_digit);
            while let Some(c) = self.peek_next()
                && c.is_numeric()
            {
                self.next();
                number.push(c);
            }
            // unwrap is safe- the loop breaks as soon as a non-digit char is encountered,
            // so all chars are certain to be digits.
            #[allow(clippy::unwrap_used)]
            number.parse().unwrap()
        }
        /// See the next char to be parsed, or None if the current char is the last char.
        fn peek_next(&self) -> Option<&char> {
            self.chars.peekable().peek()
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// The possible operable token types.
    pub enum Token {
        End,
        // 'd'
        DiceRollInvokation,
        // only positive ints are supported
        Number { value: u32 },
        Unknown,
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn parse_digits() {
            let mut cursor = Cursor::new("123");
            let first = cursor.next().unwrap();
            assert_eq!(cursor.eat_digits(first), 123);
        }
        #[test]
        fn get_dice_tokens() {
            let mut tokens = tokenise("1d6\n");
            assert_eq!(tokens.next().unwrap(), Number { value: 1 });
            assert_eq!(tokens.next().unwrap(), DiceRollInvokation);
            assert_eq!(tokens.next().unwrap(), Number { value: 6 });
            assert!(tokens.next().is_none());
        }
        #[test]
        fn unknown_token() {
            let mut tokens = tokenise("blah\n");
            assert_eq!(tokens.next().unwrap(), Unknown);
            assert_eq!(tokens.next().unwrap(), Unknown);
            assert_eq!(tokens.next().unwrap(), Unknown);
            assert_eq!(tokens.next().unwrap(), Unknown);
            assert!(tokens.next().is_none());
        }
    }
}

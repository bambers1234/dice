use std::fmt;
use std::error::Error;

/// An error in the command such that it cannot be tokenised.
#[derive(Clone, Debug)]
pub struct SyntaxError {
    token_stream: String,
}
impl SyntaxError {
    pub fn new(input: &str) -> Self {
        Self {
            token_stream: String::from(input),
        }
    }
}
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SYNTAX ERROR: {} is not valid dice notation.",
            self.token_stream
        )
    }
}
impl Error for SyntaxError {}

#[derive(Clone, Copy, Debug)]
pub struct ValueError {
    val: u32
}
impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {} is erroneous", self.val)
    }
}
impl Error for ValueError {}
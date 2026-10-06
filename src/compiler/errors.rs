use std::fmt;
use std::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct CannotInline {}
impl fmt::Display for CannotInline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This value cannot be determined at compile-time")
    }
}
impl Error for CannotInline {}


use super::tokens::Token;
use SyntaxError::*;
#[derive(Clone, Debug)]
pub enum SyntaxError {
    ExpectedToken{ after: String },
    Unknown{ token: Token },
}
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedToken { after } => write!(f, "Syntax error: expected token after {}", after),
            Unknown { token } => write!(f, "Unknown token: {}", token),
        }
    }
}
impl SyntaxError {
    pub fn token_not_found(after: String) -> Self {
        Self::ExpectedToken { after }
    }
    pub fn unknown_token(token: Token) -> Self {
        Self::Unknown { token }
    }
}
impl Error for SyntaxError {}
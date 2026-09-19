use std::fmt;
use std::error::Error;

#[derive(Clone, Debug)]
pub enum SyntaxError {
    Tokenising { token_stream: String },
    Parsing { token_stream: String },
}
impl SyntaxError {
    pub fn new_tokenising_error(input: &str) -> Self {
        Self::Tokenising { token_stream: String::from(input) }
    }
    pub fn new_parse_error(erroneous_tokens: &str) -> Self {
        Self::Parsing { token_stream: String::from(erroneous_tokens) }
    }
}
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tokenising { token_stream } => {
                write!(f, "SYNTAX ERROR: unknown pattern at {}", token_stream)
            },
            Self::Parsing { token_stream } => {
                write!(f, "SYNTAX ERROR: {} cannot be in that place in an expression", token_stream)
            },
        }
    }
}
impl Error for SyntaxError {}
#[derive(Clone, Copy, Debug)]
pub struct InvalidBytecode {
    byte: u8,
}
impl fmt::Display for InvalidBytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "Invalid byte in bytecode: 0x{:x}", self.byte
        )
    }
}
impl Error for InvalidBytecode {}
#[derive(Clone, Copy, Debug)]
pub enum NoParser {
    NoHeadHandler,
    NoTailHandler,
}
impl NoParser {
    fn no_head_handler() -> Self {
        Self::NoHeadHandler
    }
    fn no_tail_handler() -> Self {
        Self::NoTailHandler
    }
}
impl std::fmt::Display for NoParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "This token is in the wrong order, as there is no way to process it at this position"
        )
    }
}
impl Error for NoParser {}
#[derive(Clone, Copy, Debug)]
pub struct CannotInline {}
impl fmt::Display for CannotInline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This value cannot be determined at compile-time")
    }
}
impl Error for CannotInline {}
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
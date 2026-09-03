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
impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SYNTAX ERROR: {} is not valid dice notation.",
            self.token_stream
        )
    }
}
impl std::error::Error for SyntaxError {}

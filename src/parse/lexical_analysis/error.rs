use std::fmt;
use std::error::Error;

pub type ParseError = UnterminatedStringError;

#[derive(Debug, PartialEq)]
pub struct UnterminatedStringError {
    pub string: String,
    pub line: usize,
    pub column: usize,
}

impl UnterminatedStringError {
    pub fn new(string: String, line: usize, column: usize) -> Self {
        UnterminatedStringError {
            string,
            line,
            column,
        }
    }
}

impl Error for UnterminatedStringError { }

impl fmt::Display for UnterminatedStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unterminated string \"{}\" at line {} column {}",
            self.string,
            self.line,
            self.column,
        )
    }
}

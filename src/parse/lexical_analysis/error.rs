use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnterminatedStringError(UnterminatedStringError),
}

impl LexError {
    pub fn unterminated_string_error(
        string: String,
        line: usize,
        column: usize
    ) -> LexError {
        LexError::UnterminatedStringError(
            UnterminatedStringError::new(string, line, column)
        )
    }
}

impl Error for LexError { }

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::UnterminatedStringError(e) => write!(f, "{}", e),
        }
    }
}

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

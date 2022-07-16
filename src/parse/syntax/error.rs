use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxError {
    UnmatchedOpenListError,
    UnmatchedCloseListError,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxError::UnmatchedOpenListError => write!(f, "Unmatched Open List: Open parenthesis is missing close parenthesis"),
            SyntaxError::UnmatchedCloseListError => write!(f, "Unmatched Close List: Close parenthesis is missing open parenthesis"),
        }
    }
}

impl Error for SyntaxError { }

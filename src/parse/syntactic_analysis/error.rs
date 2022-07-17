use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxError {
    NoRootListError,
    NoSymbolsError,
    UnclosedRootListError,
    UnmatchedCloseListError,
    UnmatchedOpenListError,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: ")?;

        match self {
            SyntaxError::NoRootListError => write!(f, "No Root List. First item in input should be a parenthesis"),
            SyntaxError::NoSymbolsError => write!(f, "No Symbols. Input cannot be empty"),
            SyntaxError::UnclosedRootListError => write!(f, "Root list is unclosed. Add a closing parenthesis to it"),
            SyntaxError::UnmatchedOpenListError => write!(f, "Unmatched Open List: Open parenthesis is missing close parenthesis"),
            SyntaxError::UnmatchedCloseListError => write!(f, "Unmatched Close List: Close parenthesis is missing open parenthesis"),
        }
    }
}

impl Error for SyntaxError { }

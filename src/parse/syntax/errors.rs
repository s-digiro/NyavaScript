use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnmatchedOpenListError,
    UnmatchedCloseListError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnmatchedOpenListError => write!(f, "Unmatched Open List: Open parenthesis is missing close parenthesis"),
            Error::UnmatchedCloseListError => write!(f, "Unmatched Close List: Close parenthesis is missing open parenthesis"),
        }
    }
}

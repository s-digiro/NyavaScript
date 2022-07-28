use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxError {
    BadInfixDotNotation,
    FreeAtom,
    QuoteMissingItemError,
    UnexpectedTrailingTokensError,
    UnmatchedCloseListError,
    UnmatchedOpenListError,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: ")?;

        match self {
            SyntaxError::BadInfixDotNotation => write!(f, "Bad infix dot notation. Dot must be last or second to last item if it is in list"),
            SyntaxError::FreeAtom => write!(f, "FreeAtom: Atoms must be enclosed in a list"),
            SyntaxError::QuoteMissingItemError => write!(f, "Quote is missing an item after it"),
            SyntaxError::UnexpectedTrailingTokensError => write!(f, "Unexpected Trailing tokens"),
            SyntaxError::UnmatchedCloseListError => write!(f, "Unmatched Close List: Close parenthesis is missing open parenthesis"),
            SyntaxError::UnmatchedOpenListError => write!(f, "Unmatched Open List: Open parenthesis is missing close parenthesis"),
        }
    }
}

impl Error for SyntaxError { }

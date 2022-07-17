mod lexical_analysis;
pub use lexical_analysis::parse as lex_parse;
pub use lexical_analysis::{ Token, LexError };

mod semantic_analysis;
pub use semantic_analysis::parse as sem_parse;

mod syntactic_analysis;
pub use syntactic_analysis::parse as syn_parse;
pub use syntactic_analysis::{ Syntax, SyntaxError };

#[cfg(test)]
mod test;

use crate::value::ValRef;
use std::error::Error;
use std::fmt;

pub fn parse(text: &str) -> Result<ValRef, Box<dyn Error>> {
    let tokens = lexical_analysis::parse(text)?;
    let syntax = syntactic_analysis::parse(tokens)?;
    let ret = semantic_analysis::parse(syntax);

    Ok(ret)
}

// Eventually remove stuff below for better errors
#[derive(Debug)]
pub struct StringError(String);

impl StringError {
    pub fn new(string: String) -> Self {
        StringError(string)
    }
}

impl From<&str> for StringError {
    fn from(string: &str) -> Self {
        StringError::new(string.to_owned())
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StringError { }

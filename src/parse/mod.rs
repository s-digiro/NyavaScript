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

use crate::s_expression::SExpressionRef;
use std::error::Error;
use std::convert::From;

pub fn parse(text: &str) -> Result<SExpressionRef, ParseError> {
    let tokens = lexical_analysis::parse(text)?;
    let syntax = syntactic_analysis::parse(tokens)?;
    let ret = semantic_analysis::parse(syntax);

    Ok(ret)
}

#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Syntax(SyntaxError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::Lex(err) => write!(f, "{}", err),
            ParseError::Syntax(err) => write!(f, "{}", err),
        }
    }
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        ParseError::Lex(err)
    }
}

impl From<SyntaxError> for ParseError {
    fn from(err: SyntaxError) -> Self {
        ParseError::Syntax(err)
    }
}

impl Error for ParseError { }

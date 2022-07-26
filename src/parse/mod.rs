mod lexical_analysis;
pub use lexical_analysis::parse as lex_parse;
pub use lexical_analysis::{ Token, LexError };

mod semantic_analysis;
pub use semantic_analysis::parse as sem_parse;
pub use semantic_analysis::SemanticError;

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
    let ret = semantic_analysis::parse(syntax)?;

    Ok(ret)
}

#[derive(Debug)]
pub enum ParseError {
    Lex(LexError),
    Semantic(SemanticError),
    Syntax(SyntaxError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Lex(err) => write!(f, "{}", err),
            Self::Syntax(err) => write!(f, "{}", err),
            Self::Semantic(err) => write!(f, "{}", err),
        }
    }
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        Self::Lex(err)
    }
}

impl From<SyntaxError> for ParseError {
    fn from(err: SyntaxError) -> Self {
        Self::Syntax(err)
    }
}

impl From<SemanticError> for ParseError {
    fn from(err: SemanticError) -> Self {
        Self::Semantic(err)
    }
}

impl Error for ParseError { }

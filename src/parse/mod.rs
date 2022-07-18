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

pub fn parse(text: &str) -> Result<SExpressionRef, Box<dyn Error>> {
    let tokens = lexical_analysis::parse(text)?;
    let syntax = syntactic_analysis::parse(tokens)?;
    let ret = semantic_analysis::parse(syntax);

    Ok(ret)
}

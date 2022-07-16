mod lexical;
mod data;
mod syntax;
mod semantic;

use crate::expression::ExRef;

pub fn parse(text: &str) -> Result<ExRef, String> {
    semantic::parse(
        syntax::parse(
            lexical::parse(text)
        ).unwrap()
    )
}

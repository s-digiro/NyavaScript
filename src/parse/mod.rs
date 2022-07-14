mod lexical;

mod data;
pub use data::*;

mod syntax;
pub use syntax::Error as SyntaxError;

mod semantic;

use crate::data::list::ExRef;

pub fn parse(text: &str) -> Result<ExRef, String> {
    semantic::parse(
        syntax::parse(
            lexical::parse(text)
        ).unwrap()
    )
}

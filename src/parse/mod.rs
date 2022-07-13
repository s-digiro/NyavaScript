mod lexical;

mod data;
pub use data::*;

mod syntax;
pub use syntax::Error as SyntaxError;

mod semantic;

pub fn parse(text: &str) -> Result<Expression, String> {
    semantic::parse(
        syntax::parse(
            lexical::parse(text)
        ).unwrap()
    )
}

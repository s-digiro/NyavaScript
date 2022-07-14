pub mod parse;
pub mod evaluate;
pub mod data;

pub use parse::parse;
pub use parse::Expression;
pub use evaluate::evaluate;
pub use evaluate::Environment;

pub fn run(text: &str) -> Result<Expression, String> {
    let parsed = parse(text)?;

    let mut context = Environment::new();

    Ok(evaluate(&parsed, &mut context))
}

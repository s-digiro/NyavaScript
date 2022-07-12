pub mod parse;
pub mod evaluate;

pub use parse::parse;
pub use parse::Expression;
pub use evaluate::evaluate;
pub use evaluate::Context;

pub fn run(text: &str) -> Result<Expression, String> {
    let parsed = parse(text)?;

    let mut context = evaluate::Context::new();

    Ok(evaluate(parsed, &mut context))
}

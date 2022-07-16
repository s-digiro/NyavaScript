pub mod parse;
pub mod evaluate;
pub mod expression;

use expression::ExRef;
use evaluate::{ Environment, evaluate };
use parse::parse;

pub fn run(text: &str) -> Result<ExRef, String> {
    let parsed = parse(text)?;

    let mut context = Environment::new();

    Ok(evaluate(parsed, &mut context))
}

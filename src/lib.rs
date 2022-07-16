pub mod parse;
pub mod evaluate;
pub mod expression;

use expression::ExRef;
use evaluate::{ Environment, evaluate, McCarthyScope };
use parse::parse;

pub fn run(text: &str) -> Result<ExRef, String> {
    let parsed = parse(text)?;

    let mut context = Environment::new();
    context.push(McCarthyScope::new());

    Ok(evaluate(parsed, &mut context))
}

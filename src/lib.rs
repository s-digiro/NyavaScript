pub mod evaluate;
pub mod s_expression;
pub mod parse;

use evaluate::{ Environment, evaluate, McCarthyScope, FunScope };
use s_expression::SExpressionRef;
use parse::parse;
use std::error::Error;

pub fn run(text: &str) -> Result<SExpressionRef, Box<dyn Error>> {
    let parsed = parse(text)?;

    let mut context = Environment::new();
    context.push(McCarthyScope::new());
    context.push(FunScope::new());

    Ok(evaluate(parsed, &mut context))
}

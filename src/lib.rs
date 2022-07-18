pub mod evaluate;
pub mod value;
pub mod parse;

use evaluate::{ Environment, evaluate, McCarthyScope, FunScope };
use value::ValRef;
use parse::parse;
use std::error::Error;

pub fn run(text: &str) -> Result<ValRef, Box<dyn Error>> {
    let parsed = parse(text)?;

    let mut context = Environment::new();
    context.push(McCarthyScope::new());
    context.push(FunScope::new());

    Ok(evaluate(parsed, &mut context))
}

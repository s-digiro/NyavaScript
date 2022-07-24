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

#[cfg(test)]
mod test {
    use super::*;
    use crate::s_expression::SExpressionRef as SXRef;

    #[test]
    fn pipe() {
        let code = "(|> '(1 2) (cdr) (cons 4) (quote))";

        let expected = SXRef::from(vec![
            SXRef::number(4),
            SXRef::number(2),
        ]);

        let actual = run(code).unwrap();

        assert_eq!(expected, actual);
    }
}

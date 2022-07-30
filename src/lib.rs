pub mod evaluate;
pub mod s_expression;
pub mod parse;

use evaluate::{ Environment, eval_all, McCarthyScope, FunScope };
use s_expression::SExpressionRef;
use parse::parse;
use std::error::Error;

pub fn run(text: &str) -> Result<SExpressionRef, Box<dyn Error>> {
    let parsed = parse(text)?;

    let mut context = Environment::new();
    context.push_lib(McCarthyScope::new());
    context.push_lib(FunScope::new());

    let ret = eval_all(parsed, &mut context)?;

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::s_expression::{
        SExpressionRef as SXRef,
        ConsCell,
    };

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

    #[test]
    fn list_fns() {
        let code = "(cons (car '(1 2)) (cdr '(3 4)))";

        let expected = SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(4),
        ]);

        let actual = run(code).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn dot_notation() {
        let code = "(cons (car '(1 . 2)) (cdr '(3 . 4)))";

        let expected = SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::number(4),
        ));

        let actual = run(code).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_root_list() {
        let code = "

(defun foo (x y) (cons (car x) (cdr y)))
(foo '(1 2) '(3 4))

            ";

        let expected = SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(4),
        ]);

        let actual = run(code).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn comments() {
        let code = "
// This code has some comments in it!

/* This function returns a new list from
 * the first item of one list and the second
 * item of another */
(defun foo (x y) // Takes two arguments
    (cons /* Combine both args */
         (car x) // Get first from x
         (cdr y))) // Get second from y

// Test it out on (1 2) and (3 4)
(foo '(1 2) '(3 4))
/* I hope it works!
            ";

        let expected = SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(4),
        ]);

        let actual = run(code).unwrap();

        assert_eq!(expected, actual);
    }
}

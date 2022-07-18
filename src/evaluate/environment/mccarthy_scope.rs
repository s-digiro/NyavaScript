use super::Scope;
use crate::evaluate::evaluate;
use crate::s_expression::{ SExpression, Lambda, List, RustLambda, RustMacro };

pub struct McCarthyScope;

impl McCarthyScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "cons".to_string(),
            RustLambda::from(
                |e, _| {
                    List::cons(
                        &List::car(&e),
                        &List::car(&List::cdr(&e)),
                    )
                }
            )
        );

        ret.insert(
            "car".to_string(),
            RustLambda::from(
                |e, _| {
                    List::car(&List::car(&e))
                }
            )
        );

        ret.insert(
            "cdr".to_string(),
            RustLambda::from(
                |e, _| {
                    List::cdr(&List::car(&e))
                }
            )
        );

        ret.insert(
            "atom".to_string(),
            RustLambda::from(
                |e, _| {
                    let arg = List::car(&e);

                    match *arg {
                        SExpression::String(_)
                        | SExpression::Symbol(_)
                        | SExpression::Number(_)
                        | SExpression::Nil => SExpression::number(1),
                        _ => SExpression::nil(),
                    }
                }
            )
        );

        ret.insert(
            "equal".to_string(),
            RustLambda::from(
                |e, _| {
                    let arg1 = List::car(&e);
                    let arg2 = List::car(&List::cdr(&e));

                    if arg1 == arg2 {
                        SExpression::number(1)
                    } else {
                        SExpression::nil()
                    }
                }
            )
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::from(
                |e, _| {
                    SExpression::lambda(Lambda::new(e))
                }
            )
        );

        ret.insert(
            "cond".to_string(),
            RustMacro::from(
                |e, env| {
                    for arg in List::iter(&e) {
                        let p = List::car(&arg);
                        let e = List::car(&List::cdr(&arg));

                        if SExpression::nil() != evaluate(p, env) {
                            return e
                        }
                    }

                    SExpression::nil()
                }
            )
        );

        ret.insert(
            "quote".into(),
            RustMacro::from(
                |args, _| {
                    SExpression::quote(List::car(&args))
                }
            )
        );

        ret.insert(
            "null".to_string(),
            Lambda::from("(lambda (x) (equal x ()))"),
        );

        ret.insert(
            "and".to_string(),
            Lambda::from("(lambda (p q) (cond (p q)))"),
        );

        ret.insert(
            "or".to_string(),
            Lambda::from("(lambda (p q) (cond (p 1) (q 1)))"),
        );

        ret.insert(
            "not".to_string(),
            Lambda::from("(lambda (x) (cond (x ()) (1 1)))"),
        );

        ret.insert(
            "defun".to_string(),
            RustMacro::from(
                |args, env| {
                    if let SExpression::Symbol(name) = &*List::car(&args) {
                        eprintln!("name: {}", name);
                        let rest = List::cdr(&args);

                        env.set(
                            name.into(),
                            SExpression::lambda(Lambda::new(rest)),
                        );
                    }

                    SExpression::nil()
                }
            )
        );

        ret
    }
}

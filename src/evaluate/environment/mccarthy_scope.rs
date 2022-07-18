use super::Scope;
use crate::evaluate::evaluate;
use crate::value::{ Value, Lambda, List, RustLambda, RustMacro };

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
                        Value::String(_)
                        | Value::Symbol(_)
                        | Value::Number(_)
                        | Value::Nil => Value::number(1),
                        _ => Value::nil(),
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
                        Value::number(1)
                    } else {
                        Value::nil()
                    }
                }
            )
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::from(
                |e, _| {
                    Value::lambda(Lambda::new(e))
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

                        if Value::nil() != evaluate(p, env) {
                            return e
                        }
                    }

                    Value::nil()
                }
            )
        );

        ret.insert(
            "quote".into(),
            RustMacro::from(
                |args, _| {
                    Value::quote(List::car(&args))
                }
            )
        );

        ret.insert(
            "null".to_string(),
            Lambda::from("((x) (equal x ()))"),
        );

        ret.insert(
            "and".to_string(),
            Lambda::from("((p q) (cond (p q)))"),
        );

        ret.insert(
            "or".to_string(),
            Lambda::from("((p q) (cond (p 1) (q 1)))"),
        );

        ret.insert(
            "not".to_string(),
            Lambda::from("((x) (cond (x ()) (1 1)))"),
        );

        ret
    }
}

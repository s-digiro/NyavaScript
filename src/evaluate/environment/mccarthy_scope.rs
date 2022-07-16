use super::Scope;
use crate::evaluate::evaluate;
use crate::expression::{ Atom, ExRef, Lambda, List, RustLambda, RustMacro };

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

                    if arg.is_atom() || arg.is_nil() {
                        ExRef::atom(Atom::Number(1))
                    } else {
                        ExRef::nil()
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
                        ExRef::atom(Atom::Number(1))
                    } else {
                        ExRef::nil()
                    }
                }
            )
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::from(
                |e, _| {
                    ExRef::lambda(Lambda::new(&e))
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

                        if ExRef::nil() != evaluate(p, env) {
                            return e
                        }
                    }

                    ExRef::nil()
                }
            )
        );

        ret.insert(
            "null".to_string(),
            Lambda::from("(x) (equal x ())"),
        );

        ret.insert(
            "and".to_string(),
            Lambda::from("(p q) (cond (p q))"),
        );

        ret.insert(
            "or".to_string(),
            Lambda::from("(p q) (cond (p 1) (q 1))"),
        );

        ret.insert(
            "not".to_string(),
            Lambda::from("(x) (cond (x ()) (1 1))"),
        );

        ret
    }
}

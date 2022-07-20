use super::Scope;
use crate::evaluate::evaluate;
use crate::s_expression::{
    SExpression as SX,
    SExpressionRef as SXRef,
    Function,
    List,
    RustFunction,
    RustMacro
};

pub struct McCarthyScope;

impl McCarthyScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "cons".to_string(),
            RustFunction::from(
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
            RustFunction::from(
                |e, _| {
                    List::car(&List::car(&e))
                }
            )
        );

        ret.insert(
            "cdr".to_string(),
            RustFunction::from(
                |e, _| {
                    List::cdr(&List::car(&e))
                }
            )
        );

        ret.insert(
            "atom".to_string(),
            RustFunction::from(
                |e, _| {
                    let arg = List::car(&e);

                    match *arg {
                        SX::String(_)
                        | SX::Symbol(_)
                        | SX::Number(_)
                        | SX::Nil => SXRef::number(1),
                        _ => SXRef::nil(),
                    }
                }
            )
        );

        ret.insert(
            "equal".to_string(),
            RustFunction::from(
                |e, _| {
                    let arg1 = List::car(&e);
                    let arg2 = List::car(&List::cdr(&e));

                    if arg1 == arg2 {
                        SXRef::number(1)
                    } else {
                        SXRef::nil()
                    }
                }
            )
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::from(
                |e, _| {
                    SXRef::function(Function::new(e))
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

                        if SXRef::nil() != evaluate(p, env) {
                            return e
                        }
                    }

                    SXRef::nil()
                }
            )
        );

        ret.insert(
            "quote".into(),
            RustMacro::from(
                |args, _| {
                    SXRef::quote(List::car(&args))
                }
            )
        );

        ret.insert(
            "null".to_string(),
            Function::try_from("(lambda (x) (equal x ()))").unwrap().sxref(),
        );

        ret.insert(
            "and".to_string(),
            Function::try_from("(lambda (p q) (cond (p q)))").unwrap().sxref(),
        );

        ret.insert(
            "or".to_string(),
            Function::try_from("(lambda (p q) (cond (p 1) (q 1)))")
                .unwrap()
                .sxref(),
        );

        ret.insert(
            "not".to_string(),
            Function::try_from("(lambda (x) (cond (x ()) (1 1)))").unwrap()
                .sxref(),
        );

        ret.insert(
            "defun".to_string(),
            RustMacro::from(
                |args, env| {
                    if let SX::Symbol(name) = &*List::car(&args) {
                        eprintln!("name: {}", name);
                        let rest = List::cdr(&args);

                        env.set(
                            name.into(),
                            SXRef::function(Function::new(rest)),
                        );
                    }

                    SXRef::nil()
                }
            )
        );

        ret
    }
}

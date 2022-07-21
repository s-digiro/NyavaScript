use super::Scope;
use crate::evaluate::evaluate;
use crate::s_expression::{
    SExpression as SX,
    SExpressionRef as SXRef,
    Function,
    list,
    RustFunction,
    RustMacro
};

pub struct McCarthyScope;

impl McCarthyScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "cons".to_string(),
            RustFunction::new(
                |e, _| {
                    list::cons(
                        &list::car(&e),
                        &list::car(&list::cdr(&e)),
                    )
                }
            ).into(),
        );

        ret.insert(
            "car".to_string(),
            RustFunction::new(
                |e, _| {
                    list::car(&list::car(&e))
                }
            ).into(),
        );

        ret.insert(
            "cdr".to_string(),
            RustFunction::new(
                |e, _| {
                    list::cdr(&list::car(&e))
                }
            ).into(),
        );

        ret.insert(
            "atom".to_string(),
            RustFunction::new(
                |e, _| {
                    let arg = list::car(&e);

                    match *arg {
                        SX::String(_)
                        | SX::Symbol(_)
                        | SX::Number(_)
                        | SX::Nil => SXRef::number(1),
                        _ => SXRef::nil(),
                    }
                }
            ).into()
        );

        ret.insert(
            "equal".to_string(),
            RustFunction::new(
                |e, _| {
                    let arg1 = list::car(&e);
                    let arg2 = list::car(&list::cdr(&e));

                    if arg1 == arg2 {
                        SXRef::number(1)
                    } else {
                        SXRef::nil()
                    }
                }
            ).into()
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::new(
                |e, _| {
                    SXRef::function(e.into())
                }
            ).into()
        );

        ret.insert(
            "cond".to_string(),
            RustMacro::new(
                |e, env| {
                    for arg in e.iter() {
                        let p = list::car(&arg);
                        let e = list::car(&list::cdr(&arg));

                        if SXRef::nil() != evaluate(p, env) {
                            return e
                        }
                    }

                    SXRef::nil()
                }
            ).into()
        );

        ret.insert(
            "quote".into(),
            RustMacro::new(
                |args, _| {
                    SXRef::quote(list::car(&args))
                }
            ).into()
        );

        ret.insert(
            "null".to_string(),
            Function::try_from("(lambda (x) (equal x ()))").unwrap().into(),
        );

        ret.insert(
            "and".to_string(),
            Function::try_from("(lambda (p q) (cond (p q)))").unwrap().into(),
        );

        ret.insert(
            "or".to_string(),
            Function::try_from("(lambda (p q) (cond (p 1) (q 1)))")
                .unwrap()
                .into(),
        );

        ret.insert(
            "not".to_string(),
            Function::try_from("(lambda (x) (cond (x ()) (1 1)))").unwrap()
                .into(),
        );

        ret.insert(
            "defun".to_string(),
            RustMacro::new(
                |args, env| {
                    if let SX::Symbol(name) = &*list::car(&args) {
                        eprintln!("name: {}", name);
                        let rest = list::cdr(&args);

                        env.set(
                            name.into(),
                            SXRef::function(rest.into()),
                        );
                    }

                    SXRef::nil()
                }
            ).into(),
        );

        ret
    }
}

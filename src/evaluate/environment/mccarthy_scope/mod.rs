#[cfg(test)]
mod test;

use super::{
    Environment as Env,
    Scope,
};
use crate::evaluate::evaluate;
use crate::s_expression::{
    Function,
    RustFunction,
    RustMacro,
    SExpression as SX,
    SExpressionRef as SXRef,
    util,
};

pub struct McCarthyScope;

impl McCarthyScope {
    pub fn atom(args: SXRef, _env: &mut Env) -> SXRef {
        let arg = util::car(&args);

        match *arg {
            SX::String(_)
            | SX::Symbol(_)
            | SX::Number(_)
            | SX::Nil => SXRef::number(1),
            _ => SXRef::nil(),
        }
    }

    pub fn car(args: SXRef, _env: &mut Env) -> SXRef {
        util::car(&util::car(&args))
    }

    pub fn cond(sx: SXRef, env: &mut Env) -> SXRef {
        for arg in sx.iter() {
            let p = util::car(&arg);
            let e = util::car(&util::cdr(&arg));

            if SXRef::nil() != evaluate(p, env) {
                return e
            }
        }

        SXRef::nil()
    }

    pub fn cons(args: SXRef, _env: &mut Env) -> SXRef {
        util::cons(
            &util::car(&args),
            &util::car(&util::cdr(&args)),
        )
    }

    pub fn cdr(args: SXRef, _env: &mut Env) -> SXRef {
        util::cdr(&util::car(&args))
    }

    pub fn defun(sx: SXRef, env: &mut Env) -> SXRef {
        if let SX::Symbol(name) = &*util::car(&sx) {
            let rest = util::cdr(&sx);

            env.set(
                name.into(),
                SXRef::function(rest.into()),
            );
        }

        SXRef::nil()
    }

    pub fn equal(args: SXRef, _env: &mut Env) -> SXRef {
        let arg1 = util::car(&args);
        let arg2 = util::car(&util::cdr(&args));

        if arg1 == arg2 {
            SXRef::number(1)
        } else {
            SXRef::nil()
        }
    }

    pub fn lambda(sx: SXRef, _env: &mut Env) -> SXRef {
        SXRef::function(sx.into())
    }

    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "cons".to_string(),
            RustFunction::new(Self::cons).into(),
        );

        ret.insert(
            "car".to_string(),
            RustFunction::new(Self::car).into(),
        );

        ret.insert(
            "cdr".to_string(),
            RustFunction::new(Self::cdr).into(),
        );

        ret.insert(
            "atom".to_string(),
            RustFunction::new(Self::atom).into()
        );

        ret.insert(
            "equal".to_string(),
            RustFunction::new(Self::equal).into()
        );

        ret.insert(
            "lambda".to_string(),
            RustMacro::new(Self::lambda).into()
        );

        ret.insert(
            "cond".to_string(),
            RustMacro::new(Self::cond).into()
        );

        ret.insert(
            "quote".into(),
            RustMacro::new(Self::quote).into()
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
            RustMacro::new(Self::defun).into(),
        );

        ret
    }

    pub fn quote(sx: SXRef, _env: &mut Env) -> SXRef {
        SXRef::quote(util::car(&sx))
    }
}

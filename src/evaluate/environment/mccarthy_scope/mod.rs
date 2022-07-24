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

static AND: &'static str = "(lambda (p q) (cond (p q)))";
static NOT: &'static str = "(lambda (x) (cond (x ()) (1 1)))";
static NULL: &'static str = "(lambda (x) (equal x ()))";
static OR: &'static str = "(lambda (p q) (cond (p p) (q q)))";

pub struct McCarthyScope;

impl McCarthyScope {
    pub fn atom(args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
        match args.get(0) {
            None => SXRef::number(1),
            Some(sx) => match **sx {
                SX::String(_)
                | SX::Symbol(_)
                | SX::Number(_)
                | SX::Nil => SXRef::number(1),
                _ => SXRef::nil(),
            }
        }
    }

    pub fn car(args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
        match args.get(0) {
            Some(sx) => util::car(&sx),
            None => SXRef::nil(),
        }
    }

    pub fn cdr(args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
        match args.get(0) {
            Some(sx) => util::cdr(&sx),
            None => SXRef::nil(),
        }
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

    pub fn cons(args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
        let nil = SXRef::nil();

        let arg1 = args.get(0).unwrap_or(&nil);
        let arg2 = args.get(1).unwrap_or(&nil);

        util::cons(
            &arg1,
            &arg2,
        )
    }

    pub fn defun(sx: SXRef, env: &mut Env) -> SXRef {
        let mut macro_args = sx.iter();

        let _ = macro_args.next(); // skip arg 0

        let name = macro_args.next()
            .map(|sx| match &*sx {
                SX::Symbol(s) => Some(s.to_owned()),
                _ => None,
            }).flatten();

        let args = macro_args.next() // 2nd arg
            .unwrap_or(SXRef::nil())
            .iter()
            .filter_map(|sx| match &*sx {
                SX::Symbol(s) => Some(s.to_owned()),
                _ => None,
            }).collect();

        let definition = macro_args.next().unwrap_or(SXRef::nil()); // arg 3

        if let Some(name) = name {
            let f = SXRef::function(Function::new(args, definition));

            env.defun(name.into(), f);
        }

        SXRef::nil()
    }

    pub fn equal(args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
        let nil = SXRef::nil();
        let arg1 = args.get(0).unwrap_or(&nil);
        let arg2 = args.get(1).unwrap_or(&nil);

        match (&**arg1, &**arg2) {
            (SX::Function(_), _)
            | (_, SX::Function(_))
            | (SX::Macro(_), _)
            | (_, SX::Macro(_))
            | (SX::RustFunction(_), _)
            | (_, SX::RustFunction(_))
            | (SX::RustMacro(_), _)
            | (_, SX::RustMacro(_)) => SXRef::nil(),
            _ => {
                if arg1 == arg2 {
                    SXRef::number(1)
                } else {
                    SXRef::nil()
                }
            },
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
            Function::try_from(NULL).unwrap().into(),
        );

        ret.insert(
            "and".to_string(),
            Function::try_from(AND).unwrap().into(),
        );

        ret.insert(
            "or".to_string(),
            Function::try_from(OR).unwrap().into(),
        );

        ret.insert(
            "not".to_string(),
            Function::try_from(NOT).unwrap().into(),
        );

        ret.insert(
            "defun".to_string(),
            RustMacro::new(Self::defun).into(),
        );

        ret
    }

    pub fn quote(sx: SXRef, _env: &mut Env) -> SXRef {
        let arg1 = util::car(&util::cdr(&sx));

        SXRef::quote(arg1)
    }
}

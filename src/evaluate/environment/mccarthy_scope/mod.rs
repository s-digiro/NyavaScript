#[cfg(test)]
mod test;

use crate::evaluate::{
    eval,
    eval_all,
    Environment as Env,
    HashScope,
    Result as EvalResult,
};
use crate::s_expression::{
    Function,
    RustFunction,
    RustMacro,
    LabelFunction,
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
    pub fn atom(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let ret = match args.get(0) {
            None => SXRef::number(1),
            Some(sx) => match **sx {
                SX::String(_)
                | SX::Symbol(_)
                | SX::Number(_)
                | SX::Nil => SXRef::number(1),
                _ => SXRef::nil(),
            }
        };

        Ok(ret)
    }

    pub fn car(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let ret = match args.get(0) {
            Some(sx) => util::car(&sx),
            None => SXRef::nil(),
        };

        Ok(ret)
    }

    pub fn cdr(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let ret = match args.get(0) {
            Some(sx) => util::cdr(&sx),
            None => SXRef::nil(),
        };

        Ok(ret)
    }

    pub fn cond(sx: SXRef, env: &mut Env) -> EvalResult {
        for arg in sx.iter() {
            let p = util::car(&arg);
            let e = util::car(&util::cdr(&arg));

            if SXRef::nil() != eval(p, env)? {
                return Ok(e)
            }
        }

        Ok(SXRef::nil())
    }

    pub fn cons(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let nil = SXRef::nil();

        let arg1 = args.get(0).unwrap_or(&nil);
        let arg2 = args.get(1).unwrap_or(&nil);

        let ret = util::cons(&arg1, &arg2);

        Ok(ret)
    }

    pub fn defun(sx: SXRef, env: &mut Env) -> EvalResult {
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
            let f = Function::lisp_function(args, definition);

            env.defun(name.into(), f.into());
        }

        Ok(SXRef::nil())
    }

    pub fn equal(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let nil = SXRef::nil();
        let arg1 = args.get(0).unwrap_or(&nil);
        let arg2 = args.get(1).unwrap_or(&nil);

        let ret = match (&**arg1, &**arg2) {
            (SX::Function(_), _)
            | (_, SX::Function(_))
            | (SX::Macro(_), _)
            | (_, SX::Macro(_)) => SXRef::nil(),
            _ => {
                if arg1 == arg2 {
                    SXRef::number(1)
                } else {
                    SXRef::nil()
                }
            },
        };

        Ok(ret)
    }

    pub fn eval(args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        eval_all(args, env)
    }

    pub fn label(sx: SXRef, _env: &mut Env) -> EvalResult {
        Ok(SXRef::function(LabelFunction::from(sx).into()))
    }

    pub fn lambda(sx: SXRef, _env: &mut Env) -> EvalResult {
        Ok(SXRef::function(sx.into()))
    }

    pub fn list(sx: SXRef, _env: &mut Env) -> EvalResult {
        let args = sx.iter().skip(1).collect::<Vec<SXRef>>();
        let ret = SXRef::from(args);

        Ok(ret)
    }

    pub fn new() -> HashScope {
        let mut ret = HashScope::new();

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
            RustFunction::new(Self::atom).into(),
        );

        ret.insert(
            "eval".into(),
            RustFunction::new(Self::eval).into(),
        );

        ret.insert(
            "list".to_string(),
            RustMacro::new(Self::list).into(),
        );

        ret.insert(
            "NIL".to_string(),
            SXRef::nil(),
        );

        ret.insert(
            "T".into(),
            SXRef::number(1),
        );

        ret.insert(
            "equal".to_string(),
            RustFunction::new(Self::equal).into(),
        );

        ret.insert(
            "label".to_string(),
            RustMacro::new(Self::label).into()
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

    pub fn quote(sx: SXRef, _env: &mut Env) -> EvalResult {
        let arg1 = util::car(&util::cdr(&sx));

        Ok(SXRef::quote(arg1))
    }
}

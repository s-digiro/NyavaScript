#[cfg(test)]
mod test;

use crate::evaluate::{
    eval,
    eval_all,
    Environment as Env,
    Scope,
    HashScope,
    Result as EvalResult,
};
use crate::s_expression::{
    ConsCell,
    Function,
    LabelFunction,
    LispFunction,
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

#[derive(Debug)]
pub struct McCarthyScope {
    map: HashScope,
}

impl Scope for McCarthyScope {
    fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    fn entries(&self) -> Vec<(&str, &SXRef)> {
        self.map.entries()
    }

    fn get(&self, key: &str) -> Option<SXRef> {
        match self.map.get(key) {
            None => to_cadr(key),
            sx => sx.map(|sx| SXRef::clone(sx)),
        }
    }

    fn keys(&self) -> Vec<&str> {
        Scope::keys(&self.map)
    }

    fn insert(&mut self, key: String, val: SXRef) {
        Scope::insert(&mut self.map, key, val)
    }

    fn remove(&mut self, key: &str) -> Option<SXRef> {
        self.map.remove(key)
    }

    fn vals(&self) -> Vec<&SXRef> {
        self.map.vals()
    }
}

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
            let f = Function::lisp_function(args, definition, env);

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

    pub fn lambda(sx: SXRef, env: &mut Env) -> EvalResult {
        let args = util::car(&util::cdr(&sx))
            .iter()
            .filter_map(|sx| match &*sx {
                SX::Symbol(sym) => Some(sym.to_string()),
                _ => None,
            }).collect::<Vec<String>>();

        let def = util::car(&util::cdr(&util::cdr(&sx)));

        Ok(SXRef::function(Function::lisp_function(
            args,
            def,
            env,
        )))
    }

    pub fn list(sx: SXRef, _env: &mut Env) -> EvalResult {
        let args = sx.iter().skip(1).collect::<Vec<SXRef>>();
        let ret = SXRef::from(args);

        Ok(ret)
    }

    pub fn new() -> Self {
        let mut map = HashScope::new();

        map.insert(
            "cons".to_string(),
            RustFunction::new(Self::cons).into(),
        );

        map.insert(
            "car".to_string(),
            RustFunction::new(Self::car).into(),
        );

        map.insert(
            "cdr".to_string(),
            RustFunction::new(Self::cdr).into(),
        );

        map.insert(
            "atom".to_string(),
            RustFunction::new(Self::atom).into(),
        );

        map.insert(
            "eval".into(),
            RustFunction::new(Self::eval).into(),
        );

        map.insert(
            "list".to_string(),
            RustMacro::new(Self::list).into(),
        );

        map.insert(
            "NIL".to_string(),
            SXRef::nil(),
        );

        map.insert(
            "T".into(),
            SXRef::number(1),
        );

        map.insert(
            "equal".to_string(),
            RustFunction::new(Self::equal).into(),
        );

        map.insert(
            "label".to_string(),
            RustMacro::new(Self::label).into()
        );

        map.insert(
            "lambda".to_string(),
            RustMacro::new(Self::lambda).into()
        );

        map.insert(
            "cond".to_string(),
            RustMacro::new(Self::cond).into()
        );

        map.insert(
            "quote".into(),
            RustMacro::new(Self::quote).into()
        );

        map.insert(
            "null".to_string(),
            Function::try_from(NULL).unwrap().into(),
        );

        map.insert(
            "and".to_string(),
            Function::try_from(AND).unwrap().into(),
        );

        map.insert(
            "or".to_string(),
            Function::try_from(OR).unwrap().into(),
        );

        map.insert(
            "not".to_string(),
            Function::try_from(NOT).unwrap().into(),
        );

        map.insert(
            "defun".to_string(),
            RustMacro::new(Self::defun).into(),
        );

        Self {
            map,
        }
    }

    pub fn quote(sx: SXRef, _env: &mut Env) -> EvalResult {
        let arg1 = util::car(&util::cdr(&sx));

        Ok(SXRef::quote(arg1))
    }
}

fn to_cadr(s: &str) -> Option<SXRef> {
    let mut chars = s.chars();

    if let Some('c') = chars.next() {
    } else {
        return None
    }

    if let Some('r') = chars.nth_back(0) {
    } else {
        return None
    }

    let mut def = SXRef::symbol("x".into());

    for c in chars.rev() {
        match c {
            'a' => def = ConsCell::new(
                SXRef::symbol("car".into()),
                ConsCell::new(
                    def,
                    SXRef::nil(),
                ).into(),
            ).into(),
            'd' => def = ConsCell::new(
                SXRef::symbol("cdr".into()),
                ConsCell::new(
                    def,
                    SXRef::nil(),
                ).into(),
            ).into(),
            _ => return None,
        }
    }

    let ret = LispFunction::new(
        vec!["x".into()],
        def,
        &mut Env::new(),
    ).into();

    Some(ret)
}

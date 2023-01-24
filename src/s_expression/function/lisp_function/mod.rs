#[cfg(test)]
mod test;

use crate::evaluate::{
    Environment as Env,
    HashScope,
    eval,
    Result as EvalResult,
};
use crate::parse::{ parse, ParseError };
use crate::s_expression::{
    SExpression as SX,
    SExpressionRef as SXRef,
    util,
};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct LispFunction {
    args: Vec<String>,
    definition: SXRef,
    closure: HashScope,
}

impl LispFunction {
    pub fn new(args: Vec<String>, definition: SXRef, env: &mut Env) -> LispFunction {
        let pairs = capture_symbols(SXRef::clone(&definition), &args, env);
        let closure = pairs.into_iter().collect();

        LispFunction { args, definition, closure }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn definition(&self) -> SXRef {
        SXRef::clone(&self.definition)
    }

    pub fn execute(&self, args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        env.push(HashScope::new());

        for (key, val) in self.args().iter().zip(args.into_iter()) {
            env.set(key.to_owned(), val);
        }

        let ret = eval(self.definition(), env);

        env.pop();

        ret
    }
}

impl TryFrom<&str> for LispFunction {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut ast = parse(text)?;

        Ok(LispFunction::from(ast.remove(0)))
    }
}

impl From<SXRef> for LispFunction {
    fn from(sx: SXRef) -> Self {
        let args = util::car(&util::cdr(&sx)).iter()
            .filter_map(|sx| sx.as_symbol().map(|s| s.into()))
            .collect();

        let definition = util::car(&util::cdr(&util::cdr(&sx)));

        let closure = capture_symbols(
            SXRef::clone(&definition),
            &args,
            &mut Env::new(),
        ).into_iter().collect();

        LispFunction { args, definition, closure, }
    }
}

impl std::fmt::Display for LispFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[LispFunction]")
    }
}

fn capture_symbols(f: SXRef, filter: &Vec<String>, env: &mut Env) -> Vec<(String, SXRef)> {
    let ret = match &*f {
        SX::Symbol(sym) if !filter.contains(sym) =>
            vec![(sym.into(), env.get(sym))],
        SX::ConsCell(l) => l.iter()
            .map(|sx| capture_symbols(sx, filter, env))
            .flatten().collect(),
        _ => Vec::new(),
    };

    ret
}

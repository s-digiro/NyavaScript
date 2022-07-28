#[cfg(test)]
mod test;

use crate::parse::{ parse, ParseError };
use crate::s_expression::{ SExpressionRef as SXRef, util };
use crate::evaluate::{
    Environment as Env,
    Scope,
    eval,
    Result as EvalResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LispMacro {
    definition: SXRef,
    args: Vec<String>,
}

impl LispMacro {
    pub fn new(args: Vec<String>, definition: SXRef) -> LispMacro {
        LispMacro {
            definition,
            args,
        }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn definition(&self) -> SXRef {
        SXRef::clone(&self.definition)
    }

    pub fn execute(&self, args: SXRef, env: &mut Env) -> EvalResult {
        env.push(Scope::new());

        if let Some(key) = self.args().first() {
            env.set(key.to_owned(), args);
        }

        let ret = eval(self.definition(), env);

        env.pop();

        ret
    }
}

impl TryFrom<&str> for LispMacro {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut ast = parse(text)?;

        Ok(ast.remove(0).into())
    }
}

impl From<SXRef> for LispMacro {
    fn from(sx: SXRef) -> Self {
        let args = util::car(&util::cdr(&sx)).iter()
            .filter_map(|sx| sx.as_symbol().map(|s| s.into()))
            .collect();

        let definition = util::car(&util::cdr(&util::cdr(&sx)));

        LispMacro::new(args, definition)
    }
}

impl std::fmt::Display for LispMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[LispMacro]")
    }
}

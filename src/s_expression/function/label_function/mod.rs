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
use super::LispFunction;

#[derive(Clone, Debug, PartialEq)]
pub struct LabelFunction {
    label: Option<String>,
    function: SXRef,
}

impl LabelFunction {
    pub fn new(
        label: Option<String>,
        function: SXRef,
    ) -> LabelFunction {
        LabelFunction { label, function }
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    pub fn function(&self) -> &SXRef {
        &self.function
    }

    pub fn execute(&self, args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        let mut scope = HashScope::new();

        if let Some(label) = &self.label {
            scope.insert(label.into(), self.function.clone().into());
        }

        env.push(scope);

        let ret = if let SX::Function(f) = &*self.function {
            println!("FUNCTION");
            f.execute(args, env)
        } else {
            println!("NOT FUNCTION");
            eval(SXRef::clone(&self.function), env)
        };

        env.pop();

        ret
    }
}

impl TryFrom<&str> for LabelFunction {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut ast = parse(text)?;

        let ast = ast.remove(0);

        let label = util::car(&util::cdr(&ast))
            .as_symbol()
            .map(|s| s.to_owned());

        let function = util::car(&util::cdr(&util::cdr(&ast)));

        let function: LispFunction = LispFunction::from(function);

        let ret = Self::new(
            label,
            function.into()
        );

        Ok(ret)
    }
}

impl From<SXRef> for LabelFunction {
    fn from(sx: SXRef) -> Self {
        let label = util::car(&util::cdr(&sx))
            .as_symbol()
            .map(|s| s.to_owned());

        let function: SXRef = util::car(&util::cdr(&util::cdr(&sx))).into();

        let function = LispFunction::from(function).into();

        println!("{:?}", function);

        LabelFunction { label, function }
    }
}

impl std::fmt::Display for LabelFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[LabelFunction]")
    }
}

use crate::evaluate::{
    Environment as Env,
    Result as EvalResult,
};
use crate::s_expression::SExpressionRef as SXRef;

pub type MacroFunc = fn(SXRef, &mut Env) -> EvalResult;

#[derive(Clone)]
pub struct RustMacro(MacroFunc);

impl RustMacro {
    pub fn new(f: MacroFunc) -> RustMacro {
        RustMacro(f)
    }

    pub fn execute(&self, list: SXRef, env: &mut Env) -> EvalResult {
        self.0(list, env)
    }
}

impl From<MacroFunc> for RustMacro {
    fn from(f: MacroFunc) -> Self {
        Self::new(f)
    }
}

impl PartialEq for RustMacro {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Do not call PartialEq on a RustMacro.");
    }
}

impl std::fmt::Debug  for RustMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustMacro]")
    }
}

impl std::fmt::Display for RustMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustMacro]")
    }
}

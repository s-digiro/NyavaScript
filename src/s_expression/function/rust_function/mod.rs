#[cfg(test)]
mod test;

use crate::evaluate::Environment;
use crate::s_expression::SExpressionRef as SXRef;

pub type ScopeableRustFn = fn(Vec<SXRef>, &mut Environment) -> SXRef;

#[derive(Clone)]
pub struct RustFunction(ScopeableRustFn);

impl RustFunction {
    pub fn new(f: ScopeableRustFn) -> RustFunction {
        RustFunction(f)
    }

    pub fn execute(&self, args: Vec<SXRef>, env: &mut Environment) -> SXRef {
        self.0(args, env)
    }
}

impl PartialEq for RustFunction {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Do not call PartialEq on a RustFunction.");
    }
}

impl std::fmt::Debug  for RustFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustFunction]")
    }
}

impl std::fmt::Display for RustFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustFunction]")
    }
}

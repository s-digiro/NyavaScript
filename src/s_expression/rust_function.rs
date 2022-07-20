use crate::evaluate::Environment;
use crate::s_expression::SExpressionRef as SXRef;

type Func = fn(SXRef, &mut Environment) -> SXRef;

pub struct RustFunction(Func);

impl RustFunction {
    pub fn new(f: Func) -> RustFunction {
        RustFunction(f)
    }

    pub fn exec(&self, list: SXRef, env: &mut Environment) -> SXRef {
        self.0(list, env)
    }

    pub fn from(f: Func) -> SXRef {
        SXRef::rust_function(RustFunction::new(f))
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

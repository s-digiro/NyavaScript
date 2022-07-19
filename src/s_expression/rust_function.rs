use crate::evaluate::Environment;
use crate::s_expression::{ SExpression, SExpressionRef };

type Func = fn(SExpressionRef, &mut Environment) -> SExpressionRef;

pub struct RustFunction(Func);

impl RustFunction {
    pub fn new(f: Func) -> RustFunction {
        RustFunction(f)
    }

    pub fn exec(&self, list: SExpressionRef, env: &mut Environment) -> SExpressionRef {
        self.0(list, env)
    }

    pub fn from(f: Func) -> SExpressionRef {
        SExpressionRef::rust_function(RustFunction::new(f))
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

use crate::evaluate::Environment;
use crate::s_expression::{ SExpression, SExpressionRef };

type LambdaFunc = fn(SExpressionRef, &mut Environment) -> SExpressionRef;

pub struct RustLambda(LambdaFunc);

impl RustLambda {
    pub fn new(f: LambdaFunc) -> RustLambda {
        RustLambda(f)
    }

    pub fn exec(&self, list: SExpressionRef, env: &mut Environment) -> SExpressionRef {
        self.0(list, env)
    }

    pub fn from(f: LambdaFunc) -> SExpressionRef {
        SExpression::rust_lambda(RustLambda::new(f))
    }
}

impl PartialEq for RustLambda {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Do not call PartialEq on a RustLambda.");
    }
}

impl std::fmt::Debug  for RustLambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustLambda]")
    }
}

impl std::fmt::Display for RustLambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[RustLambda]")
    }
}

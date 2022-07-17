use crate::evaluate::Environment;
use crate::expression::{ Value, ValRef };

type LambdaFunc = fn(ValRef, &mut Environment) -> ValRef;

pub struct RustLambda(LambdaFunc);

impl RustLambda {
    pub fn new(f: LambdaFunc) -> RustLambda {
        RustLambda(f)
    }

    pub fn exec(&self, list: ValRef, env: &mut Environment) -> ValRef {
        self.0(list, env)
    }

    pub fn from(f: LambdaFunc) -> ValRef {
        Value::rust_lambda(RustLambda::new(f))
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

use crate::evaluate::Environment;
use crate::expression::{ ConsCell, ExRef };

pub struct RustLambda(fn(&ConsCell, &mut Environment) -> ExRef);

impl RustLambda {
    pub fn exec(&self, list: &ConsCell, env: &mut Environment) -> ExRef {
        self.0(list, env)
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

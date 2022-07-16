use crate::evaluate::Environment;
use crate::expression::{ ConsCell, ExRef };

pub struct RustMacro(fn(&ConsCell, &mut Environment) -> ExRef);

impl RustMacro {
    pub fn exec(&self, list: &ConsCell, env: &mut Environment) -> ExRef {
        self.0(list, env)
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

use crate::evaluate::Environment;
use crate::expression::{ Value, ValRef };

type MacroFunc = fn(ValRef, &mut Environment) -> ValRef;

pub struct RustMacro(MacroFunc);

impl RustMacro {
    pub fn new(f: MacroFunc) -> RustMacro {
        RustMacro(f)
    }

    pub fn exec(&self, list: ValRef, env: &mut Environment) -> ValRef {
        self.0(list, env)
    }

    pub fn from(f: MacroFunc) -> ValRef {
        Value::rust_macro(RustMacro::new(f))
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

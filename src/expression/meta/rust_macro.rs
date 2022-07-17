use crate::evaluate::Environment;
use crate::expression::ExRef;

type MacroFunc = fn(ExRef, &mut Environment) -> ExRef;

pub struct RustMacro(MacroFunc);

impl RustMacro {
    pub fn new(f: MacroFunc) -> RustMacro {
        RustMacro(f)
    }

    pub fn exec(&self, list: ExRef, env: &mut Environment) -> ExRef {
        self.0(list, env)
    }

    pub fn from(f: MacroFunc) -> ExRef {
        ExRef::rust_macro(RustMacro::new(f))
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

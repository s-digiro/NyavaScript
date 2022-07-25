use crate::s_expression::SExpressionRef as SXRef;
use crate::evaluate::Environment as Env;
use crate::parse::ParseError;

mod lisp_macro;
pub use lisp_macro::*;

mod rust_macro;
pub use rust_macro::*;

#[derive(Clone)]
pub enum Macro {
    Lisp(LispMacro),
    Rust(RustMacro),
}

impl Macro {
    pub fn execute(&self, sx: SXRef, env: &mut Env) -> SXRef {
        match self {
            Self::Lisp(f) => f.execute(sx, env),
            Self::Rust(f) => f.execute(sx, env),
        }
    }

    pub fn lisp_macro(args: Vec<String>, definition: SXRef) -> Macro {
        Self::Lisp(LispMacro::new(args, definition))
    }

    pub fn rust_macro(f: MacroFunc) -> Macro {
        Self::Rust(RustMacro::new(f))
    }
}

impl PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lisp(l), Self::Lisp(o)) => l.eq(o),
            (Self::Rust(r), Self::Rust(o)) => r.eq(o),
            _ => false,
        }
    }
}

impl std::fmt::Debug  for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Lisp(l) => write!(f, "{:?}", l),
            Self::Rust(r) => write!(f, "{:?}", r),
        }
    }
}

impl std::fmt::Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Lisp(l) => write!(f, "{}", l),
            Self::Rust(r) => write!(f, "{}", r),
        }
    }
}

impl TryFrom<&str> for Macro {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let f = text.try_into()?;

        Ok(Self::Lisp(f))
    }
}

impl From<SXRef> for Macro {
    fn from(sx: SXRef) -> Self {
        Self::Lisp(sx.into())
    }
}

impl From<RustMacro> for Macro {
    fn from(f: RustMacro) -> Self {
        Self::Rust(f)
    }
}

impl From<LispMacro> for Macro {
    fn from(l: LispMacro) -> Self {
        Self::Lisp(l)
    }
}

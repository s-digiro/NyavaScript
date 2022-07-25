mod lisp_function;
pub use lisp_function::LispFunction;

mod rust_function;
pub use rust_function::ScopeableRustFn;
pub use rust_function::RustFunction;

use crate::s_expression::SExpressionRef as SXRef;
use crate::evaluate::Environment as Env;
use crate::parse::ParseError;

#[derive(Clone)]
pub enum Function {
    Lisp(LispFunction),
    Rust(RustFunction),
}

impl Function {
    pub fn execute(&self, args: Vec<SXRef>, env: &mut Env) -> SXRef {
        match self {
            Self::Lisp(f) => f.execute(args, env),
            Self::Rust(f) => f.execute(args, env),
        }
    }

    pub fn lisp_function(args: Vec<String>, definition: SXRef) -> Function {
        Self::Lisp(LispFunction::new(args, definition))
    }

    pub fn rust_function(f: RustFunction) -> Self {
        Self::Rust(f)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Lisp(l), Self::Lisp(o)) => l.eq(o),
            (Self::Rust(r), Self::Rust(o)) => r.eq(o),
            _ => false,
        }
    }
}

impl std::fmt::Debug  for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Lisp(l) => write!(f, "{:?}", l),
            Self::Rust(r) => write!(f, "{:?}", r),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Lisp(l) => write!(f, "{}", l),
            Self::Rust(r) => write!(f, "{}", r),
        }
    }
}

impl TryFrom<&str> for Function {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let f = text.try_into()?;

        Ok(Self::Lisp(f))
    }
}

impl From<SXRef> for Function {
    fn from(sx: SXRef) -> Self {
        Self::Lisp(sx.into())
    }
}

impl From<RustFunction> for Function {
    fn from(f: RustFunction) -> Self {
        Self::Rust(f)
    }
}

impl From<LispFunction> for Function {
    fn from(l: LispFunction) -> Self {
        Self::Lisp(l)
    }
}

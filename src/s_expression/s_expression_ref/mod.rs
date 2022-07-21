mod iter;
pub use iter::*;

#[cfg(test)]
mod test;

use std::rc::Rc;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct SExpressionRef(Rc<SExpression>);

use std::ops::Deref;

impl Deref for SExpressionRef {
    type Target = SExpression;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl SExpressionRef {
    pub fn new(sx: SExpression) -> Self {
        Self(Rc::new(sx))
    }

    pub fn clone(sx: &Self) -> Self {
        Self(Rc::clone(&sx.0))
    }

    pub fn cons_cell(c: ConsCell) -> Self {
        Self::new(SExpression::ConsCell(c))
    }

    pub fn function(function: Function) -> Self {
        Self::new(SExpression::Function(function))
    }

    pub fn iter(&self) -> ListIter {
        ListIter {
            current: Self::clone(self),
        }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn r#macro(m: Macro) -> Self {
        Self::new(SExpression::Macro(m))
    }

    pub fn number(n: isize) -> Self {
        Self::new(SExpression::Number(n))
    }

    pub fn nil() -> Self {
        Self::new(SExpression::Nil)
    }

    pub fn quote(v: Self) -> Self {
        Self::new(SExpression::Quote(v))
    }

    pub fn rust_function(f: RustFunction) -> Self {
        Self::new(SExpression::RustFunction(f))
    }

    pub fn rust_macro(m: RustMacro) -> Self {
        Self::new(SExpression::RustMacro(m))
    }

    pub fn string(s: String) -> Self {
        Self::new(SExpression::String(s))
    }

    pub fn symbol(s: String) -> Self {
        Self::new(SExpression::Symbol(s))
    }
}

impl std::fmt::Display for SExpressionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Function> for SExpressionRef {
    fn from(f: Function) -> Self {
        Self::function(f)
    }
}

impl From<RustFunction> for SExpressionRef {
    fn from(f: RustFunction) -> Self {
        Self::rust_function(f)
    }
}

impl From<RustMacro> for SExpressionRef {
    fn from(f: RustMacro) -> Self {
        Self::rust_macro(f)
    }
}

impl From<Vec<Self>> for SExpressionRef {
    fn from(v: Vec<Self>) -> Self {
        let mut ret = Self::nil();

        for e in v.into_iter().rev() {
            ret = util::cons(&e, &ret)
        }

        ret
    }
}

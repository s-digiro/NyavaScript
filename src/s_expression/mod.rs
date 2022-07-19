mod cons_cell;
pub use cons_cell::ConsCell;

mod function;
pub use function::Function;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Macro, RustLambda, RustMacro };

use enum_as_inner::EnumAsInner;
use std::rc::Rc;

#[derive(Debug, PartialEq, EnumAsInner)]
pub enum SExpression {
    // Basic
    ConsCell(ConsCell),
    Number(isize),
    String(String),
    Symbol(String),
    Nil,

    // Higher level abstraction
    Function(Function),
    Macro(Macro),
    RustLambda(RustLambda),
    RustMacro(RustMacro),
    Quote(SExpressionRef),
}

impl std::fmt::Display for SExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SExpression::ConsCell(x) => write!(f, "{}", x),
            SExpression::Number(x) => write!(f, "{}", x),
            SExpression::String(x) => write!(f, "\"{}\"", x),
            SExpression::Symbol(x) => write!(f, "{}", x),
            SExpression::Nil => write!(f, "NIL"),
            SExpression::Function(x) => write!(f, "{}", x),
            SExpression::Macro(x) => write!(f, "{}", x),
            SExpression::RustLambda(x) => write!(f, "{}", x),
            SExpression::RustMacro(x) => write!(f, "{}", x),
            SExpression::Quote(x) => write!(f, "'{}", x),
        }
    }
}

impl SExpression {
    pub fn from_iter<I: IntoIterator<Item=SExpressionRef>>(i: I) -> SExpressionRef {
        let mut ret = SExpressionRef::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for valref in i.into_iter().collect::<Vec<SExpressionRef>>().iter().rev() {
            ret = List::cons(valref, &ret);
        }

        ret
    }
}

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

    pub fn rust_lambda(lambda: RustLambda) -> Self {
        Self::new(SExpression::RustLambda(lambda))
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

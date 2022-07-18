mod cons_cell;
pub use cons_cell::ConsCell;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Lambda, Macro, RustLambda, RustMacro };

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
    Lambda(Lambda),
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
            SExpression::Lambda(x) => write!(f, "{}", x),
            SExpression::Macro(x) => write!(f, "{}", x),
            SExpression::RustLambda(x) => write!(f, "{}", x),
            SExpression::RustMacro(x) => write!(f, "{}", x),
            SExpression::Quote(x) => write!(f, "'{}", x),
        }
    }
}

pub type SExpressionRef = Rc<SExpression>;

impl SExpression {
    pub fn cons_cell(c: ConsCell) -> SExpressionRef {
        Rc::new(SExpression::ConsCell(c))
    }

    pub fn lambda(lambda: Lambda) -> SExpressionRef {
        Rc::new(SExpression::Lambda(lambda))
    }

    pub fn r#macro(m: Macro) -> SExpressionRef {
        Rc::new(SExpression::Macro(m))
    }

    pub fn number(n: isize) -> SExpressionRef {
        Rc::new(SExpression::Number(n))
    }

    pub fn nil() -> SExpressionRef {
        Rc::new(SExpression::Nil)
    }

    pub fn quote(v: SExpressionRef) -> SExpressionRef {
        Rc::new(SExpression::Quote(v))
    }

    pub fn rust_lambda(lambda: RustLambda) -> SExpressionRef {
        Rc::new(SExpression::RustLambda(lambda))
    }

    pub fn rust_macro(m: RustMacro) -> SExpressionRef {
        Rc::new(SExpression::RustMacro(m))
    }

    pub fn string(s: String) -> SExpressionRef {
        Rc::new(SExpression::String(s))
    }

    pub fn symbol(s: String) -> SExpressionRef {
        Rc::new(SExpression::Symbol(s))
    }

    pub fn from_iter<I: IntoIterator<Item=SExpressionRef>>(i: I) -> SExpressionRef {
        let mut ret = SExpression::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for valref in i.into_iter().collect::<Vec<SExpressionRef>>().iter().rev() {
            ret = List::cons(valref, &ret);
        }

        ret
    }
}

mod cons_cell;
pub use cons_cell::ConsCell;

mod function;
pub use function::Function;

mod r#macro;
pub use r#macro::Macro;

mod rust_function;
pub use rust_function::RustFunction;

mod rust_macro;
pub use rust_macro::RustMacro;

mod s_expression_ref;
pub use s_expression_ref::{ SExpressionRef, ListIter };

pub mod util;

#[derive(Debug, PartialEq)]
pub enum SExpression {
    ConsCell(ConsCell),
    Function(Function),
    Macro(Macro),
    Nil,
    Number(isize),
    Quote(SExpressionRef),
    RustFunction(RustFunction),
    RustMacro(RustMacro),
    String(String),
    Symbol(String),
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
            SExpression::RustFunction(x) => write!(f, "{}", x),
            SExpression::RustMacro(x) => write!(f, "{}", x),
            SExpression::Quote(x) => write!(f, "'{}", x),
        }
    }
}

impl SExpression {
    pub fn as_cons_cell(&self) -> Option<&ConsCell> {
        match self {
            Self::ConsCell(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn from_iter<I: IntoIterator<Item=SExpressionRef>>(i: I) -> SExpressionRef {
        let mut ret = SExpressionRef::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for valref in i.into_iter().collect::<Vec<SExpressionRef>>().iter().rev() {
            ret = util::cons(valref, &ret);
        }

        ret
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }
}

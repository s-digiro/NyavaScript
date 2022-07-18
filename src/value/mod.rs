mod cons_cell;
pub use cons_cell::ConsCell;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Lambda, Macro, RustLambda, RustMacro };

use enum_as_inner::EnumAsInner;
use std::rc::Rc;

#[derive(Debug, PartialEq, EnumAsInner)]
pub enum Value {
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
    Quote(ValRef),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::ConsCell(x) => write!(f, "{}", x),
            Value::Number(x) => write!(f, "{}", x),
            Value::String(x) => write!(f, "\"{}\"", x),
            Value::Symbol(x) => write!(f, "{}", x),
            Value::Nil => write!(f, "NIL"),
            Value::Lambda(x) => write!(f, "{}", x),
            Value::Macro(x) => write!(f, "{}", x),
            Value::RustLambda(x) => write!(f, "{}", x),
            Value::RustMacro(x) => write!(f, "{}", x),
            Value::Quote(x) => write!(f, "'{}", x),
        }
    }
}

pub type ValRef = Rc<Value>;

impl Value {
    pub fn cons_cell(c: ConsCell) -> ValRef {
        Rc::new(Value::ConsCell(c))
    }

    pub fn lambda(lambda: Lambda) -> ValRef {
        Rc::new(Value::Lambda(lambda))
    }

    pub fn r#macro(m: Macro) -> ValRef {
        Rc::new(Value::Macro(m))
    }

    pub fn number(n: isize) -> ValRef {
        Rc::new(Value::Number(n))
    }

    pub fn nil() -> ValRef {
        Rc::new(Value::Nil)
    }

    pub fn quote(v: ValRef) -> ValRef {
        Rc::new(Value::Quote(v))
    }

    pub fn rust_lambda(lambda: RustLambda) -> ValRef {
        Rc::new(Value::RustLambda(lambda))
    }

    pub fn rust_macro(m: RustMacro) -> ValRef {
        Rc::new(Value::RustMacro(m))
    }

    pub fn string(s: String) -> ValRef {
        Rc::new(Value::String(s))
    }

    pub fn symbol(s: String) -> ValRef {
        Rc::new(Value::Symbol(s))
    }

    pub fn from_iter<I: IntoIterator<Item=ValRef>>(i: I) -> ValRef {
        let mut ret = Value::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for valref in i.into_iter().collect::<Vec<ValRef>>().iter().rev() {
            ret = List::cons(valref, &ret);
        }

        ret
    }
}

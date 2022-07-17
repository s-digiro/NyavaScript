mod atom;
pub use atom::Atom;

mod cons_cell;
pub use cons_cell::ConsCell;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Lambda, Macro, RustLambda, RustMacro };

use derive_new::new;
use enum_as_inner::EnumAsInner;
use std::rc::Rc;
use std::fmt::Display;
use enum_display_derive::Display;

#[derive(Debug, PartialEq, EnumAsInner, Display, new)]
pub enum Value {
    // Basic
    ConsCell(ConsCell),
    Number(isize),
    String(String),
    Symbol(String),

    // Higher level abstraction
    Lambda(Lambda),
    Macro(Macro),
    RustLambda(RustLambda),
    RustMacro(RustMacro),
}

#[derive(Debug, PartialEq)]
pub struct ValRef(Option<Rc<Value>>);

impl ValRef {
    pub fn as_cons_cell(&self) -> Option<&ConsCell> {
        self.0.as_ref().map(|e| e.as_cons_cell()).flatten()
    }

    pub fn as_number(&self) -> Option<&isize> {
        self.0.as_ref().map(|e| e.as_number()).flatten()
    }

    pub fn as_string(&self) -> Option<&String> {
        self.0.as_ref().map(|e| e.as_string()).flatten()
    }

    pub fn as_symbol(&self) -> Option<&String> {
        self.0.as_ref().map(|e| e.as_symbol()).flatten()
    }

    pub fn as_lambda(&self) -> Option<&Lambda> {
        self.0.as_ref().map(|e| e.as_lambda()).flatten()
    }

    pub fn as_macro(&self) -> Option<&Macro> {
        self.0.as_ref().map(|e| e.as_macro()).flatten()
    }

    pub fn as_rust_lambda(&self) -> Option<&RustLambda> {
        self.0.as_ref().map(|e| e.as_rust_lambda()).flatten()
    }

    pub fn as_rust_macro(&self) -> Option<&RustMacro> {
        self.0.as_ref().map(|e| e.as_rust_macro()).flatten()
    }

    pub fn clone(e: &ValRef) -> ValRef {
        match &e.0 {
            Some(rc) => ValRef(Some(Rc::clone(&rc))),
            None => ValRef(None),
        }
    }

    pub fn cons_cell(c: ConsCell) -> ValRef {
        ValRef::new(Value::ConsCell(c))
    }

    pub fn is_cons_cell(&self) -> bool {
        self.as_cons_cell().is_some()
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    pub fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    pub fn is_symbol(&self) -> bool {
        self.as_symbol().is_some()
    }

    pub fn is_lambda(&self) -> bool {
        self.as_lambda().is_some()
    }

    pub fn is_macro(&self) -> bool {
        self.as_macro().is_some()
    }

    pub fn is_rust_lambda(&self) -> bool {
        self.as_rust_lambda().is_some()
    }

    pub fn is_rust_macro(&self) -> bool {
        self.as_rust_macro().is_some()
    }

    pub fn lambda(lambda: Lambda) -> ValRef {
        ValRef::new(Value::Lambda(lambda))
    }

    pub fn r#macro(m: Macro) -> ValRef {
        ValRef::new(Value::Macro(m))
    }

    pub fn new(v: Value) -> ValRef {
        ValRef(Some(Rc::new(v)))
    }

    pub fn number(n: isize) -> ValRef {
        ValRef::new(Value::Number(n))
    }

    pub fn nil() -> ValRef {
        ValRef(None)
    }

    pub fn rust_lambda(lambda: RustLambda) -> ValRef {
        ValRef::new(Value::RustLambda(lambda))
    }

    pub fn rust_macro(m: RustMacro) -> ValRef {
        ValRef::new(Value::RustMacro(m))
    }

    pub fn string(s: String) -> ValRef {
        ValRef::new(Value::String(s))
    }

    pub fn symbol(s: String) -> ValRef {
        ValRef::new(Value::Symbol(s))
    }
}

impl FromIterator<ValRef> for ValRef {
    fn from_iter<I: IntoIterator<Item=ValRef>>(i: I) -> Self {
        let mut ret = ValRef::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for valref in i.into_iter().collect::<Vec<ValRef>>().iter().rev() {
            ret = List::cons(valref, &ret);
        }

        ret
    }
}

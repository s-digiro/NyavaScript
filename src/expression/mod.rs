mod atom;
pub use atom::Atom;

mod cons_cell;
pub use cons_cell::ConsCell;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Lambda, Macro, RustLambda, RustMacro };

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Expression {
    ConsCell(ConsCell),
    Atom(Atom),

    // Higher level abstraction
    Lambda(Lambda),
    Macro(Macro),
    RustLambda(RustLambda),
    RustMacro(RustMacro),
}

impl Expression {
    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Expression::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_cons_cell(&self) -> Option<&ConsCell> {
        match self {
            Expression::ConsCell(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&ConsCell> {
        self.as_cons_cell()
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Expression::Atom(_) => true,
            _ => false,
        }
    }

    pub fn is_cons_cell(&self) -> bool {
        match self {
            Expression::ConsCell(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        self.is_cons_cell()
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::ConsCell(c) => write!(f, "{}", c),
            Expression::Atom(a) => write!(f, "{}", a),
            Expression::Lambda(l) => write!(f, "{}", l),
            Expression::Macro(m) => write!(f, "{}", m),
            Expression::RustLambda(l) => write!(f, "{}", l),
            Expression::RustMacro(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ExRef(Option<Rc<Expression>>);

impl ExRef {
    pub fn as_atom(&self) -> Option<&Atom> {
        self.0.as_ref().map(|e| e.as_atom()).flatten()
    }

    pub fn as_cons_cell(&self) -> Option<&ConsCell> {
        self.0.as_ref().map(|e| e.as_cons_cell()).flatten()
        //self.0.map(|e| e.as_cons_cell()).flatten()
    }

    pub fn as_expression(&self) -> Option<&Expression> {
        self.0.as_ref().map(|e| &**e)
    }

    pub fn as_list(&self) -> Option<&ConsCell> {
        self.as_cons_cell()
    }

    pub fn atom(a: Atom) -> ExRef {
        ExRef(Some(Rc::new(Expression::Atom(a))))
    }

    pub fn cons_cell(c: ConsCell) -> ExRef {
        ExRef(Some(Rc::new(Expression::ConsCell(c))))
    }

    pub fn clone(e: &ExRef) -> ExRef {
        match &e.0 {
            Some(rc) => ExRef(Some(Rc::clone(&rc))),
            None => ExRef(None),
        }
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_none()
    }

    pub fn is_atom(&self) -> bool {
        self.0.as_ref().map(|e| e.is_atom()).unwrap_or(false)
    }

    pub fn is_cons_cell(&self) -> bool {
        self.0.as_ref().map(|e| e.is_cons_cell()).unwrap_or(false)
    }

    pub fn is_list(&self) -> bool {
        self.is_cons_cell()
    }

    pub fn lambda(lambda: Lambda) -> ExRef {
        ExRef(Some(Rc::new(Expression::Lambda(lambda))))
    }

    pub fn list(list: ConsCell) -> ExRef {
        ExRef(Some(Rc::new(Expression::ConsCell(list))))
    }

    pub fn new(e: Expression) -> ExRef {
        ExRef(Some(Rc::new(e)))
    }

    pub fn nil() -> ExRef {
        ExRef(None)
    }

    pub fn rust_lambda(lambda: RustLambda) -> ExRef {
        ExRef(Some(Rc::new(Expression::RustLambda(lambda))))
    }

    pub fn rust_macro(m: RustMacro) -> ExRef {
        ExRef(Some(Rc::new(Expression::RustMacro(m))))
    }
}

impl std::fmt::Display for ExRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.as_expression() {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "NIL"),
        }
    }
}

impl FromIterator<ExRef> for ExRef {
    fn from_iter<I: IntoIterator<Item=ExRef>>(i: I) -> Self {
        let mut ret = ExRef::nil();

        // Probably a more efficient way to do this, but since cons aren't
        // double ended, just convert them to vecs for now
        for exref in i.into_iter().collect::<Vec<ExRef>>().iter().rev() {
            ret = List::cons(exref, &ret);
        }

        ret
    }
}

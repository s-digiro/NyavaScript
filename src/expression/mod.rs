mod atom;
pub use atom::Atom;

mod cons_cell;
pub use cons_cell::ConsCell;

mod list;
pub use list::List;

mod meta;
pub use meta::{ Lambda, Macro, RustLambda, RustMacro };

use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Debug, PartialEq)]
pub struct ExRef(Option<Rc<RefCell<Expression>>>);

use std::cell::Ref;

impl ExRef {
    pub fn as_atom(&self) -> Option<&Atom> {
        self.0.map(|e| e.borrow().as_atom()).flatten()
    }

    pub fn as_cons_cell(&self) -> Option<&ConsCell> {
        match self.as_expression() {
            Some(e) if e.is_cons_cell() => {
                e.as_cons_cell()
            },
            _ => None,
        }
        //self.as_expression().map(|e| if e.is_cons_cell() { 
        //self.0.map(|e| e.borrow().as_cons_cell()).flatten()
    }

    pub fn as_cons_cell2(&self) -> Option<Ref<ConsCell>> {
        match self.as_expression2() {
            Some(e) if e.is_cons_cell() => {
                Some(Ref::map(e, |e| e.as_cons_cell().unwrap()))
            },
            _ => None,
        }
    }

    pub fn as_expression(&self) -> Option<&Expression> {
        self.0.as_ref().map(|e| &*e.borrow())
    }

    pub fn as_expression2(&self) -> Option<Ref<Expression>> {
        self.0.as_ref().map(|e| e.borrow())
    }

    pub fn as_list(&self) -> Option<&ConsCell> {
        self.as_cons_cell()
    }

    pub fn atom(a: Atom) -> ExRef {
        ExRef(Some(Rc::new(RefCell::new(Expression::Atom(a)))))
    }

    pub fn cons_cell(c: ConsCell) -> ExRef {
        ExRef(Some(Rc::new(RefCell::new(Expression::ConsCell(c)))))
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
        match self.0 {
            Some(e) => e.borrow().is_atom(),
            _ => false,
        }
    }

    pub fn is_cons_cell(&self) -> bool {
        match self.0 {
            Some(e) => e.borrow().is_cons_cell(),
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        self.is_cons_cell()
    }

    pub fn list(list: ConsCell) -> ExRef {
        ExRef(Some(Rc::new(RefCell::new(Expression::ConsCell(list)))))
    }

    pub fn nil() -> ExRef {
        ExRef(None)
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

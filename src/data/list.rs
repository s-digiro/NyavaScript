use std::rc::Rc;
use std::cell::{ Ref, RefCell };

pub enum Expression {
    ConsCell(ConsCell),
    Atom(Atom),
}

struct ExRef(Option<Rc<RefCell<Expression>>>);

impl ExRef {
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

    pub fn is_cons_cell(&self) -> bool {
        match self.as_expression() {
            Some(e) => Ref::filter_map(e, |e| match e {
                Expression::ConsCell(c) => Some(c),
                _ => None,
            }).is_ok(),
            _ => false,
        }
    }

    pub fn nil() -> ExRef {
        ExRef(None)
    }

    pub fn as_expression(&self) -> Option<Ref<Expression>> {
        match &self.0 {
            Some(e) => Some(e.borrow()),
            None => None,
        }
    }

    pub fn as_cons_cell(&self) -> Option<Ref<ConsCell>> {
        match self.as_expression() {
            Some(e) => Ref::filter_map(e, |e| match e {
                Expression::ConsCell(c) => Some(c),
                _ => None,
            }).ok(),
            _ => None,
        }
    }
}

pub struct ConsCell {
    car: ExRef,
    cdr: ExRef,
}

impl ConsCell {
    pub fn nil() -> ExRef {
        ExRef::nil()
    }

    pub fn new(car: ExRef, cdr: ExRef) -> ExRef {
        ExRef::cons_cell(
            ConsCell {
                car,
                cdr,
            }
        )
    }
}

pub enum Atom {
    Symbol(String),
    String(String),
    Number(isize),
}

impl Atom {
    pub fn nil() -> ExRef {
        ExRef::nil()
    }

    pub fn symbol(s: &str) -> ExRef {
        ExRef::atom(Atom::Symbol(s.to_owned()))
    }

    pub fn string(s: &str) -> ExRef {
        ExRef::atom(Atom::String(s.to_owned()))
    }

    pub fn number(n: isize) -> ExRef {
        ExRef::atom(Atom::Number(n))
    }
}

struct List(ExRef);

impl List {
    pub fn new() -> ExRef {
        ExRef::nil()
    }

    pub fn nil() -> ExRef {
        ExRef::nil()
    }

    pub fn car(list: &ExRef) -> ExRef {
        match &list.0 {
            Some(e) => match &*e.borrow() {
                Expression::ConsCell(cc) => ExRef::clone(&cc.car),
                _ => ExRef::nil(),
            },
            None => ExRef::nil(),
        }
    }

    pub fn cdr(list: &ExRef) -> ExRef {
        match &list.0 {
            Some(e) => match &*e.borrow() {
                Expression::ConsCell(cc) => ExRef::clone(&cc.cdr),
                _ => ExRef::nil(),
            },
            None => ExRef::nil(),
        }
    }

    pub fn cons(car: &ExRef, cdr: &ExRef) -> ExRef {
        ConsCell::new(
            ExRef::clone(car),
            ExRef::clone(cdr),
        )
    }

    pub fn push(list: &mut ExRef, item: &ExRef) {
        if List::car(list).is_cons_cell() {
            List::push(&mut List::car(list), item);
        } else {
            match &list.0 {
                Some(e) => match &mut *e.borrow_mut() {
                    Expression::ConsCell(cc) => cc.car = ExRef::clone(item),
                    _ => panic!("Somehow push got called on an expression that isn't a cons list"),
                },
                None => {
                    std::mem::replace(
                        list,
                        List::cons(
                            item,
                            &List::nil(),
                        ),
                    );
                }
            }
        }
    }
}

impl IntoIterator for List {
    type Item = ExRef;
    type IntoIter = ListIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter {
            current: self.0,
        }
    }
}

pub struct ListIntoIter {
    current: ExRef,
}

impl Iterator for ListIntoIter {
    type Item = ExRef;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = List::car(&self.current);
        self.current = List::cdr(&self.current);

        Some(ret)
    }
}

use std::rc::Rc;
use std::cell::{ RefCell, Ref };

#[derive(Debug, PartialEq)]
pub enum Expression {
    ConsCell(ConsCell),
    Atom(Atom),
}

#[derive(Debug, PartialEq)]
pub struct ExRef(Option<Rc<RefCell<Expression>>>);

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
        match self.0 {
            Some(e) => match *e.borrow() {
                Expression::ConsCell(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn is_list(&self) -> bool {
        self.is_cons_cell()
    }

    pub fn nil() -> ExRef {
        ExRef(None)
    }

    pub fn as_atom(&self) -> Option<Ref<String>> {
        match self {
            Some(e) => e.borrow() {
                match e {
                    Expression::Atom(a) => Some(a),
                    _ => None,
                }
            },
            None => None,
        }
    }

}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

pub struct List(ExRef);

impl List {
    pub fn new() -> ExRef {
        ExRef::nil()
    }

    pub fn from(v: Vec<ExRef>) -> ExRef {
        let mut ret = List::new();

        for e in v.into_iter().rev() {
            ret = List::cons(&e, &ret)
        }

        ret
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

    pub fn len(list: &ExRef) -> usize {
        List(list).iter().count()
    }
}

impl <'a> IntoIterator for &'a List {
    type Item = ExRef;
    type IntoIter = ListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter {
            current: self.0,
        }
    }
}

pub struct ListIter<'a> {
    current: &'a ExRef,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = ExRef;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = List::car(&self.current);
        self.current = List::cdr(&self.current);

        Some(ret)
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

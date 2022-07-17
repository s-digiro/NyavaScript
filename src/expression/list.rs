use super::{ ConsCell, ValRef };

pub struct List(ValRef);

impl List {
    pub fn new() -> ValRef {
        ValRef::nil()
    }

    pub fn from(v: Vec<ValRef>) -> ValRef {
        let mut ret = List::new();

        for e in v.into_iter().rev() {
            ret = List::cons(&e, &ret)
        }

        ret
    }

    pub fn nil() -> ValRef {
        ValRef::nil()
    }

    pub fn car(list: &ValRef) -> ValRef {
        match list.as_cons_cell() {
            Some(e) => ValRef::clone(&e.car),
            None => ValRef::nil(),
        }
    }

    pub fn cdr(list: &ValRef) -> ValRef {
        match list.as_cons_cell() {
            Some(e) => ValRef::clone(&e.cdr),
            None => ValRef::nil(),
        }
    }

    pub fn cons(car: &ValRef, cdr: &ValRef) -> ValRef {
        ValRef::cons_cell(
            ConsCell::new(
                ValRef::clone(car),
                ValRef::clone(cdr),
            )
        )
    }

    pub fn push(list: &ValRef, item: &ValRef) -> ValRef {
        if list.is_nil() {
            List::cons(item, &List::nil())
        } else {
            let head = List::car(&list);
            let tail = List::cdr(&list);

            List::cons(
                &head,
                &List::push(&tail, item),
            )
        }
    }

    pub fn len(list: &ValRef) -> usize {
        List::iter(list).count()
    }

    pub fn iter(list: &ValRef) -> ListIter {
        ListIter {
            current: ValRef::clone(list),
        }
    }
}

pub struct ListIter {
    current: ValRef,
}

impl Iterator for ListIter {
    type Item = ValRef;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = List::car(&self.current);
        self.current = List::cdr(&self.current);

        match ret.0 {
            Some(_) => Some(ret),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expression::{ Atom, Expression };

    #[test]
    fn cons_works() {
        assert_eq!(
            ValRef::new(Expression::ConsCell(ConsCell::new(
                ValRef::new(Expression::Atom(Atom::Number(1))),
                ValRef::new(Expression::Atom(Atom::Number(2))),
            ))),
            List::cons(&ValRef::atom(Atom::Number(1)), &ValRef::atom(Atom::Number(2))),
        );
    }
}

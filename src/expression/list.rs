use super::{ ConsCell, ExRef };

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
        match list.as_cons_cell() {
            Some(e) => ExRef::clone(&e.car),
            None => ExRef::nil(),
        }
    }

    pub fn cdr(list: &ExRef) -> ExRef {
        match list.as_cons_cell() {
            Some(e) => ExRef::clone(&e.cdr),
            None => ExRef::nil(),
        }
    }

    pub fn cons(car: &ExRef, cdr: &ExRef) -> ExRef {
        ExRef::cons_cell(
            ConsCell::new(
                ExRef::clone(car),
                ExRef::clone(cdr),
            )
        )
    }

    pub fn push(list: &ExRef, item: &ExRef) -> ExRef {
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

    pub fn len(list: &ExRef) -> usize {
        List::iter(list).count()
    }

    pub fn iter(list: &ExRef) -> ListIter {
        ListIter {
            current: ExRef::clone(list),
        }
    }
}

pub struct ListIter {
    current: ExRef,
}

impl Iterator for ListIter {
    type Item = ExRef;

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
            ExRef::new(Expression::ConsCell(ConsCell::new(
                ExRef::new(Expression::Atom(Atom::Number(1))),
                ExRef::new(Expression::Atom(Atom::Number(2))),
            ))),
            List::cons(&ExRef::atom(Atom::Number(1)), &ExRef::atom(Atom::Number(2))),
        );
    }
}

use super::{ ConsCell, Expression, ExRef };

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

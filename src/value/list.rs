use super::*;

pub struct List(ValRef);

impl List {
    pub fn new() -> ValRef {
        Value::nil()
    }

    pub fn from(v: Vec<ValRef>) -> ValRef {
        let mut ret = List::new();

        for e in v.into_iter().rev() {
            ret = List::cons(&e, &ret)
        }

        ret
    }

    pub fn nil() -> ValRef {
        Value::nil()
    }

    pub fn car(list: &ValRef) -> ValRef {
        match list.as_cons_cell() {
            Some(e) => Rc::clone(&e.car),
            None => Value::nil(),
        }
    }

    pub fn cdr(list: &ValRef) -> ValRef {
        match list.as_cons_cell() {
            Some(e) => Rc::clone(&e.cdr),
            None => Value::nil(),
        }
    }

    pub fn cons(car: &ValRef, cdr: &ValRef) -> ValRef {
        Value::cons_cell(
            ConsCell::new(
                Rc::clone(car),
                Rc::clone(cdr),
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
            current: Rc::clone(list),
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

        match &*ret {
            Value::Nil => None,
            _ => Some(ret),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn cons_works() {
        assert_eq!(
            Value::cons_cell(ConsCell::new(
                Value::number(1),
                Value::number(2),
            )),
            List::cons(&Value::number(1), &Value::number(2)),
        );
    }
}

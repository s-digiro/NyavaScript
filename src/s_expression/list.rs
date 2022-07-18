use super::*;

pub struct List(SExpressionRef);

impl List {
    pub fn new() -> SExpressionRef {
        SExpression::nil()
    }

    pub fn from(v: Vec<SExpressionRef>) -> SExpressionRef {
        let mut ret = List::new();

        for e in v.into_iter().rev() {
            ret = List::cons(&e, &ret)
        }

        ret
    }

    pub fn nil() -> SExpressionRef {
        SExpression::nil()
    }

    pub fn car(list: &SExpressionRef) -> SExpressionRef {
        match list.as_cons_cell() {
            Some(e) => Rc::clone(&e.car),
            None => SExpression::nil(),
        }
    }

    pub fn cdr(list: &SExpressionRef) -> SExpressionRef {
        match list.as_cons_cell() {
            Some(e) => Rc::clone(&e.cdr),
            None => SExpression::nil(),
        }
    }

    pub fn cons(car: &SExpressionRef, cdr: &SExpressionRef) -> SExpressionRef {
        SExpression::cons_cell(
            ConsCell::new(
                Rc::clone(car),
                Rc::clone(cdr),
            )
        )
    }

    pub fn push(list: &SExpressionRef, item: &SExpressionRef) -> SExpressionRef {
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

    pub fn len(list: &SExpressionRef) -> usize {
        List::iter(list).count()
    }

    pub fn iter(list: &SExpressionRef) -> ListIter {
        ListIter {
            current: Rc::clone(list),
        }
    }
}

pub struct ListIter {
    current: SExpressionRef,
}

impl Iterator for ListIter {
    type Item = SExpressionRef;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = List::car(&self.current);
        self.current = List::cdr(&self.current);

        match &*ret {
            SExpression::Nil => None,
            _ => Some(ret),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::s_expression::SExpression;

    #[test]
    fn cons_works() {
        assert_eq!(
            SExpression::cons_cell(ConsCell::new(
                SExpression::number(1),
                SExpression::number(2),
            )),
            List::cons(&SExpression::number(1), &SExpression::number(2)),
        );
    }
}

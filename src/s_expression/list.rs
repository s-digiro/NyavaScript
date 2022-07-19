use super::SExpressionRef as SXRef;
use super::*;

pub struct List(SXRef);

impl List {
    pub fn new() -> SXRef {
        SExpressionRef::nil()
    }

    pub fn from(v: Vec<SXRef>) -> SXRef {
        let mut ret = List::new();

        for e in v.into_iter().rev() {
            ret = List::cons(&e, &ret)
        }

        ret
    }

    pub fn nil() -> SXRef {
        SExpressionRef::nil()
    }

    pub fn car(list: &SXRef) -> SXRef {
        match list.as_cons_cell() {
            Some(e) => SXRef::clone(&e.car),
            None => SExpressionRef::nil(),
        }
    }

    pub fn cdr(list: &SXRef) -> SXRef {
        match list.as_cons_cell() {
            Some(e) => SXRef::clone(&e.cdr),
            None => SExpressionRef::nil(),
        }
    }

    pub fn cons(car: &SXRef, cdr: &SXRef) -> SXRef {
        SExpressionRef::cons_cell(
            ConsCell::new(
                SXRef::clone(car),
                SXRef::clone(cdr),
            )
        )
    }

    pub fn push(list: &SXRef, item: &SXRef) -> SXRef {
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

    pub fn len(list: &SXRef) -> usize {
        List::iter(list).count()
    }

    pub fn iter(list: &SXRef) -> ListIter {
        ListIter {
            current: SXRef::clone(list),
        }
    }
}

pub struct ListIter {
    current: SXRef,
}

impl Iterator for ListIter {
    type Item = SXRef;

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
    use super::SExpressionRef as SXRef;

    #[test]
    fn cons_works() {
        assert_eq!(
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(1),
                SXRef::number(2),
            )),
            List::cons(&SXRef::number(1), &SXRef::number(2)),
        );
    }

    #[test]
    fn car_works() {
        let subject = SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::number(2),
        ));

        assert_eq!(
            SXRef::number(1),
            List::car(&subject),
        );
    }

    #[test]
    fn cdr_works() {
        let subject = SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::number(2),
        ));

        assert_eq!(
            SXRef::number(2),
            List::cdr(&subject),
        );
    }
}

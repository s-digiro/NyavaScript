use super::SExpressionRef as SXRef;
use super::*;

pub fn car(list: &SXRef) -> SXRef {
    match list.as_cons_cell() {
        Some(e) => SXRef::clone(&e.car),
        None => SXRef::nil(),
    }
}

pub fn cdr(list: &SXRef) -> SXRef {
    match list.as_cons_cell() {
        Some(e) => SXRef::clone(&e.cdr),
        None => SXRef::nil(),
    }
}

pub fn cons(car: &SXRef, cdr: &SXRef) -> SXRef {
    SXRef::cons_cell(
        ConsCell::new(
            SXRef::clone(car),
            SXRef::clone(cdr),
        )
    )
}

pub fn push(list: &SXRef, item: &SXRef) -> SXRef {
    if list.is_nil() {
        util::cons(item, &SXRef::nil())
    } else {
        let head = util::car(&list);
        let tail = util::cdr(&list);

        util::cons(
            &head,
            &util::push(&tail, item),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::SXRef as SXRef;

    #[test]
    fn cons_works() {
        assert_eq!(
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(1),
                SXRef::number(2),
            )),
            util::cons(&SXRef::number(1), &SXRef::number(2)),
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
            util::car(&subject),
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
            util::cdr(&subject),
        );
    }
}

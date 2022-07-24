#[cfg(test)]
mod test;

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
    } else if list.is_cons_cell() {
        let head = util::car(&list);
        let tail = util::cdr(&list);

        util::cons(
            &head,
            &util::push(&tail, item),
        )
    } else {
        util::cons(
            list,
            &util::cons(item, &SXRef::nil()),
        )
    }
}

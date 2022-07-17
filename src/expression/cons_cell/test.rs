use super::*;

use crate::expression::Atom;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        ValRef::atom(Atom::Number(1)),
        ValRef::cons_cell(ConsCell::new(
            ValRef::atom(Atom::Number(2)),
            ValRef::cons_cell(ConsCell::new(
                ValRef::atom(Atom::Number(3)),
                ValRef::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(ValRef::atom(Atom::Number(1))),
        iter.next(),
    );

    assert_eq!(
        Some(ValRef::atom(Atom::Number(2))),
        iter.next(),
    );

    assert_eq!(
        Some(ValRef::atom(Atom::Number(3))),
        iter.next(),
    );

    assert_eq!(
        None,
        iter.next(),
    );
}

#[test]
fn display_works() {
    let subject = ConsCell::new(
        ValRef::atom(Atom::Number(1)),
        ValRef::cons_cell(ConsCell::new(
            ValRef::atom(Atom::Number(2)),
            ValRef::cons_cell(ConsCell::new(
                ValRef::atom(Atom::Number(3)),
                ValRef::nil(),
            )),
        )),
    );

    eprintln!("{}", subject.to_string());
    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

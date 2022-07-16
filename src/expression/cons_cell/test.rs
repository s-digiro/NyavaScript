use super::*;

use crate::expression::Atom;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        ExRef::atom(Atom::Number(1)),
        ExRef::cons_cell(ConsCell::new(
            ExRef::atom(Atom::Number(2)),
            ExRef::cons_cell(ConsCell::new(
                ExRef::atom(Atom::Number(3)),
                ExRef::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(ExRef::atom(Atom::Number(1))),
        iter.next(),
    );

    assert_eq!(
        Some(ExRef::atom(Atom::Number(2))),
        iter.next(),
    );

    assert_eq!(
        Some(ExRef::atom(Atom::Number(3))),
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
        ExRef::atom(Atom::Number(1)),
        ExRef::cons_cell(ConsCell::new(
            ExRef::atom(Atom::Number(2)),
            ExRef::cons_cell(ConsCell::new(
                ExRef::atom(Atom::Number(3)),
                ExRef::nil(),
            )),
        )),
    );

    eprintln!("{}", subject.to_string());
    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

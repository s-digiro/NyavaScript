use super::*;
use crate::s_expression::SExpressionRef as SXRef;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        SXRef::number(1),
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(2),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(3),
                SXRef::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(SXRef::number(1)),
        iter.next(),
    );

    assert_eq!(
        Some(SXRef::number(2)),
        iter.next(),
    );

    assert_eq!(
        Some(SXRef::number(3)),
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
        SXRef::number(1),
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(2),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(3),
                SXRef::nil(),
            )),
        )),
    );

    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

#[test]
fn display_on_cons_cell_with_non_list_cdr_uses_dot_infix_notation() {
    panic!("FAIL");
}

#[test]
fn display_on_list_with_last_cons_cell_with_non_list_cdr_uses_dot_infix_notation() {
    panic!("FAIL");
}

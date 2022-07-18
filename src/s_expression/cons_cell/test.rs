use super::*;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        SExpression::number(1),
        SExpression::cons_cell(ConsCell::new(
            SExpression::number(2),
            SExpression::cons_cell(ConsCell::new(
                SExpression::number(3),
                SExpression::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(SExpression::number(1)),
        iter.next(),
    );

    assert_eq!(
        Some(SExpression::number(2)),
        iter.next(),
    );

    assert_eq!(
        Some(SExpression::number(3)),
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
        SExpression::number(1),
        SExpression::cons_cell(ConsCell::new(
            SExpression::number(2),
            SExpression::cons_cell(ConsCell::new(
                SExpression::number(3),
                SExpression::nil(),
            )),
        )),
    );

    eprintln!("{}", subject.to_string());
    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

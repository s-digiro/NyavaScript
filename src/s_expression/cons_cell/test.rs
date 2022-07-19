use super::*;
use crate::s_expression::SExpression as SX;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        SX::number(1),
        SX::cons_cell(ConsCell::new(
            SX::number(2),
            SX::cons_cell(ConsCell::new(
                SX::number(3),
                SX::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(SX::number(1)),
        iter.next(),
    );

    assert_eq!(
        Some(SX::number(2)),
        iter.next(),
    );

    assert_eq!(
        Some(SX::number(3)),
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
        SX::number(1),
        SX::cons_cell(ConsCell::new(
            SX::number(2),
            SX::cons_cell(ConsCell::new(
                SX::number(3),
                SX::nil(),
            )),
        )),
    );

    eprintln!("{}", subject.to_string());
    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

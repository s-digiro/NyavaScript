use super::*;

#[test]
fn iter_works() {
    let subject = ConsCell::new(
        Value::number(1),
        Value::cons_cell(ConsCell::new(
            Value::number(2),
            Value::cons_cell(ConsCell::new(
                Value::number(3),
                Value::nil(),
            )),
        )),
    );

    let mut iter = subject.iter();

    assert_eq!(
        Some(Value::number(1)),
        iter.next(),
    );

    assert_eq!(
        Some(Value::number(2)),
        iter.next(),
    );

    assert_eq!(
        Some(Value::number(3)),
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
        Value::number(1),
        Value::cons_cell(ConsCell::new(
            Value::number(2),
            Value::cons_cell(ConsCell::new(
                Value::number(3),
                Value::nil(),
            )),
        )),
    );

    eprintln!("{}", subject.to_string());
    assert_eq!(
        "(1 2 3 )",
        subject.to_string(),
    );
}

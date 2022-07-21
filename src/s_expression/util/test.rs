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
fn car_works_on_list_of_2() {
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
fn car_works_on_list_of_3() {
    let subject = SXRef::from(vec![
        SXRef::string("g".into()),
        SXRef::number(4),
        SXRef::number(1),
    ]);

    assert_eq!(
        SXRef::string("g".into()),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_list_starting_with_nil() {
    let subject = SXRef::from(vec![
        SXRef::nil(),
        SXRef::number(4),
        SXRef::number(1),
    ]);

    assert_eq!(
        SXRef::nil(),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_list_starting_with_list() {
    let subject = SXRef::from(vec![
        SXRef::from(vec![SXRef::number(2), SXRef::number(3)]),
        SXRef::number(4),
        SXRef::number(1),
    ]);

    assert_eq!(
        SXRef::from(vec![SXRef::number(2), SXRef::number(3)]),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_symbol() {
    let subject = SXRef::symbol("test".into());

    assert_eq!(
        SXRef::nil(),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_string() {
    let subject = SXRef::string("test".into());

    assert_eq!(
        SXRef::nil(),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_number() {
    let subject = SXRef::number(2);

    assert_eq!(
        SXRef::nil(),
        util::car(&subject),
    );
}

#[test]
fn car_works_on_nil() {
    let subject = SXRef::nil();

    assert_eq!(
        SXRef::nil(),
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

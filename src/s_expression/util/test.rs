use super::*;
use super::SXRef as SXRef;

#[test]
fn cons_works_on_numbers() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::number(2),
        )),
        util::cons(&SXRef::number(1), &SXRef::number(2)),
    );
}

#[test]
fn cons_number_appended_to_nil() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::nil(),
        )),
        util::cons(&SXRef::number(1), &SXRef::nil()),
    );
}

#[test]
fn cons_number_appended_to_cons_cell() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(1),
            SXRef::cons_cell(ConsCell::new(
                    SXRef::number(2),
                    SXRef::cons_cell(ConsCell::new(
                            SXRef::number(3),
                            SXRef::nil(),
                    )),
            )),
        )),
        util::cons(
            &SXRef::number(1),
            &SXRef::cons_cell(ConsCell::new(
                SXRef::number(2),
                SXRef::cons_cell(ConsCell::new(
                        SXRef::number(3),
                        SXRef::nil(),
                )),
            )),
        ),
    );
}

#[test]
fn cons_appends_list_to_list() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::cons_cell(ConsCell::new(
                    SXRef::number(1),
                    SXRef::cons_cell(ConsCell::new(
                            SXRef::number(2),
                            SXRef::nil(),
                    )),
            )),
            SXRef::cons_cell(ConsCell::new(
                    SXRef::number(3),
                    SXRef::nil(),
            )),
        )),
        util::cons(
            &SXRef::cons_cell(ConsCell::new(
                    SXRef::number(1),
                    SXRef::cons_cell(ConsCell::new(
                            SXRef::number(2),
                            SXRef::nil(),
                    )),
            )),
            &SXRef::cons_cell(ConsCell::new(
                    SXRef::number(3),
                    SXRef::nil(),
            )),
        ),
    );
}

#[test]
fn cons_can_append_nil_to_nil() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::nil(),
            SXRef::nil(),
        )),
        util::cons(
            &SXRef::nil(),
            &SXRef::nil(),
        ),
    );
}
#[test]
fn cons_can_append_nil_to_list() {
    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::nil(),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(1),
                SXRef::cons_cell(ConsCell::new(
                    SXRef::number(2),
                    SXRef::nil(),
                )),
            )),
        )),
        util::cons(
            &SXRef::nil(),
            &SXRef::cons_cell(ConsCell::new(
                SXRef::number(1),
                SXRef::cons_cell(ConsCell::new(
                    SXRef::number(2),
                    SXRef::nil(),
                )),
            )),
        ),
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
fn cdr_works_on_list_of_2() {
    let subject = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    assert_eq!(
        SXRef::number(2),
        util::cdr(&subject),
    );
}

#[test]
fn cdr_works_on_nil() {
    let subject = SXRef::nil();

    assert_eq!(
        SXRef::nil(),
        util::cdr(&subject),
    );
}

#[test]
fn cdr_works_on_list_of_1() {
    let subject = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::nil(),
    ));

    assert_eq!(
        SXRef::nil(),
        util::cdr(&subject),
    );
}

#[test]
fn cdr_works_on_list_of_3() {
    let subject = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(2),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(3),
                SXRef::nil(),
            ))
        )),
    ));

    assert_eq!(
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(2),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(3),
                SXRef::nil(),
            ))
        )),
        util::cdr(&subject),
    );
}

#[test]
fn cdr_works_on_atom() {
    let subject = SXRef::number(1);

    assert_eq!(
        SXRef::nil(),
        util::cdr(&subject),
    );
}

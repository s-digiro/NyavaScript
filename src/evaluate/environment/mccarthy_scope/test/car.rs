use super::*;

#[test]
pub fn car_returns_first_item_in_list_of_2() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ]),
    ];

    let expected =  SXRef::number(1);

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_of_1() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
        ]),
    ];

    let expected =  SXRef::number(1);

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_of_3() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3),
        ]),
    ];

    let expected =  SXRef::number(1);

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_starting_with_list() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]),
            SXRef::number(3),
        ]),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_list_starting_with_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(1),
        ]),
    ];

    let expected =  SXRef::nil();

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_atom() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
    ];

    let expected =  SXRef::nil();

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_item_in_car_slot_when_called_on_cons_cell() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(ConsCell::new(
            SXRef::number(1),
            SXRef::number(2),
        )),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::car(subject, &mut env);

    assert_eq!(expected, actual)
}

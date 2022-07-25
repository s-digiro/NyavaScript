use super::*;
use crate::s_expression::ConsCell;

#[test]
pub fn cdr_returns_nil_when_called_on_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_list_of_1_nil_when_called_on_list_of_2_nils() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::nil(),
        ]),
    ];

    let expected = SXRef::from(vec![SXRef::nil()]);

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_nil_when_called_on_atom() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_nil_when_called_on_list_of_1() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
        ]),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_list_of_last_item_when_called_on_list_of_2() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ]),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(2),
    ]);

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_list_of_2_last_items_when_called_on_list_of_3() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3),
        ]),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_item_in_cdr_slot_when_called_on_cons_cell() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(ConsCell::new(
            SXRef::number(1),
            SXRef::number(2),
        )),
    ];

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cdr(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

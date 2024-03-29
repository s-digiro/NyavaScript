use super::*;
use crate::s_expression::ConsCell;

#[test]
pub fn cons_creates_new_cons_cell_from_args() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
        SXRef::from(vec![SXRef::number(2)]),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_from_single_arg() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
        SXRef::nil(),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(1),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_from_no_arg() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
        SXRef::nil(),
    ];

    let expected = SXRef::from(vec![
        SXRef::nil(),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_first_arg_is_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
        SXRef::from(vec![SXRef::number(2)]),
    ];

    let expected = SXRef::from(vec![
        SXRef::nil(),
        SXRef::number(2),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_first_arg_is_list() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3)
        ]),
        SXRef::from(vec![SXRef::number(4)]),
    ];

    let expected = SXRef::from(vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3)
        ]),
        SXRef::number(4),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_second_arg_is_list_of_multiple_items() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
        SXRef::from(vec![
            SXRef::number(2),
            SXRef::number(3),
            SXRef::number(4)
        ]),
    ];

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
        SXRef::number(4),
    ]);

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_second_arg_is_an_atom() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
        SXRef::number(2),
    ];

    let expected = SXRef::from(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    let actual = McCarthyScope::cons(subject, &mut env).unwrap();

    assert_eq!(expected, actual)
}

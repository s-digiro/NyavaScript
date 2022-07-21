use super::*;
use crate::evaluate::{
    Environment as Env,
    evaluate as eval,
};
use crate::s_expression::{
    ConsCell,
    SExpressionRef as SXRef,
};

#[test]
pub fn cons_creates_new_cons_cell_from_args() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::number(1),
        SXRef::quote(SXRef::from(vec![SXRef::number(2)])),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_from_single_arg() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::number(1),
        SXRef::nil(),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_from_no_arg() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::nil(),
        SXRef::nil(),
    ]);

    let expected = SXRef::from(vec![
        SXRef::nil(),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_first_arg_is_nil() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::nil(),
        SXRef::quote(SXRef::from(vec![SXRef::number(2)])),
    ]);

    let expected = SXRef::from(vec![
        SXRef::nil(),
        SXRef::number(2),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_first_arg_is_list() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
                SXRef::number(3)
            ])
        ),
        SXRef::quote(SXRef::from(vec![SXRef::number(4)])),
    ]);

    let expected = SXRef::from(vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3)
        ]),
        SXRef::number(4),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_second_arg_is_list_of_multiple_items() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::number(1),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(2),
                SXRef::number(3),
                SXRef::number(4)
            ])
        ),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
        SXRef::number(4),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cons_creates_new_cons_cell_when_second_arg_is_an_atom() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let expected = SXRef::from(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_of_2() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]),
        ),
    ]);

    let expected =  SXRef::number(1);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_of_1() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(1),
            ]),
        ),
    ]);

    let expected =  SXRef::number(1);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_nil() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::nil(),
    ]);

    let expected = SXRef::nil();

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_first_item_in_list_starting_with_list() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::from(vec![
                    SXRef::number(1),
                    SXRef::number(2),
                ]),
                SXRef::number(3),
            ]),
        ),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = eval(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_list_starting_with_nil() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]),
        ),
    ]);

    let expected =  SXRef::number(1);

    let actual = eval(subject, &mut env);

    assert_eq!(true, false)
}

#[test]
pub fn car_returns_nil_from_atom() {
    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let subject = SXRef::from(vec![
        SXRef::symbol("car".into()),
        SXRef::quote(
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]),
        ),
    ]);

    let expected =  SXRef::number(1);

    let actual = eval(subject, &mut env);

    assert_eq!(true, false)
}

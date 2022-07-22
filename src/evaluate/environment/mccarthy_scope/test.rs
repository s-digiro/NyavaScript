use super::*;
use crate::evaluate::Environment as Env;
use crate::s_expression::{
    ConsCell,
    SExpressionRef as SXRef,
};

pub fn dummy_fn(_: &Vec<SXRef>, _: &mut Env) -> SXRef {
    SXRef::nil()
}

pub fn dummy_macro(_: SXRef, _: &mut Env) -> SXRef {
    SXRef::nil()
}

#[test]
pub fn atom_returns_1_when_called_on_nil() {
    let mut env = Env::new();

    let subject = vec![SXRef::nil()];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_symbol() {
    let mut env = Env::new();

    let subject = vec![SXRef::symbol("foo".into())];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_number() {
    let mut env = Env::new();

    let subject = vec![SXRef::number(1)];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_string() {
    let mut env = Env::new();

    let subject = vec![SXRef::string("foo".into())];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_list() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(1),
        ])
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_quote() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_function() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::function("()".try_into().unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_macro() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::r#macro("()".try_into().unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_rust_lambda() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::rust_function(RustFunction::new(dummy_fn))
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_rust_macro() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::rust_macro(RustMacro::new(dummy_macro)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(&subject, &mut env);

    assert_eq!(expected, actual)
}

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

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

    let actual = McCarthyScope::cons(&subject, &mut env);

    assert_eq!(expected, actual)
}

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

    let actual = McCarthyScope::car(&subject, &mut env);

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

    let actual = McCarthyScope::car(&subject, &mut env);

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

    let actual = McCarthyScope::car(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::car(&subject, &mut env);

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

    let actual = McCarthyScope::car(&subject, &mut env);

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

    let actual = McCarthyScope::car(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn car_returns_nil_from_atom() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
    ];

    let expected =  SXRef::nil();

    let actual = McCarthyScope::car(&subject, &mut env);

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

    let actual = McCarthyScope::car(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_nil_when_called_on_nil() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::cdr(&subject, &mut env);

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

    let actual = McCarthyScope::cdr(&subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn cdr_returns_nil_when_called_on_atom() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::cdr(&subject, &mut env);

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

    let actual = McCarthyScope::cdr(&subject, &mut env);

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

    let actual = McCarthyScope::cdr(&subject, &mut env);

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

    let actual = McCarthyScope::cdr(&subject, &mut env);

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

    let actual = McCarthyScope::cdr(&subject, &mut env);

    assert_eq!(expected, actual)
}

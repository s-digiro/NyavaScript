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

#[test]
pub fn cond_returns_nil_when_called_with_no_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_nil_when_only_predicate_is_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(1),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_first_val_when_first_predicate_is_true_with_1_branch() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_first_val_when_first_predicate_is_truthy_with_multi_branch() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(3),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_2nd_val_when_1st_predicate_is_nil_and_2nd_is_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(3),
        ]),
    ]);

    let expected = SXRef::number(3);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_2nd_val_when_1st_predicate_is_nil_and_2nd_and_3rd_are_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(3),
        ]),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(4),
        ]),
    ]);

    let expected = SXRef::number(3);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_3rd_val_when_1st_and_2nd_predicate_are_nil_and_3rd_is_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(3),
        ]),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(4),
        ]),
    ]);

    let expected = SXRef::number(4);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_evaluates_to_nil_when_all_3_predicates_are_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(3),
        ]),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(4),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_nil_to_be_falsey() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::nil(),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_symbol_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::quote(SXRef::symbol("foo".into())),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_blank_symbol_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::quote(SXRef::symbol("".into())),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_string_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::string("foo".into()),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_blank_string_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::string("".into()),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_number_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(15),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_zero_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(0),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_function_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::function("()".try_into().unwrap()),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_macro_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::r#macro("()".try_into().unwrap()),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_rust_function_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            RustFunction::new(dummy_fn).into(),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_rust_macro_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            RustMacro::new(dummy_macro).into(),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_list_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::quote(SXRef::from(vec![SXRef::number(1)])),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_considers_quote_to_be_truthy() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::quote(SXRef::quote(SXRef::number(1))),
            SXRef::number(2),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_returns_nil_when_truthy_predicate_does_not_have_expression_following_it() {
    panic!("FAIL")
}

#[test]
pub fn cond_returns_nil_when_truthy_predicate_has_expression_that_returns_nil() {
    panic!("FAIL")
}

#[test]
pub fn cond_returns_second_arg_expression_value_when_truthy_predicate_is_followed_by_multiple_arguments() {
    panic!("FAIL")
}

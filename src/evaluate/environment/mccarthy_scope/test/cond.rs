use super::*;

#[test]
pub fn cond_returns_nil_when_called_with_no_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

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

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_returns_nil_when_truthy_predicate_does_not_have_expression_following_it() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(1),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_returns_nil_when_truthy_predicate_has_expression_that_returns_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::nil(),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn cond_returns_second_arg_expression_value_when_truthy_predicate_is_followed_by_multiple_arguments() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cond".into()),
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
            SXRef::number(3),
        ]),
    ]);

    let expected = SXRef::number(2);

    let actual = McCarthyScope::cond(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

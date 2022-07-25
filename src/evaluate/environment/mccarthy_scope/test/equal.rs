use super::*;
use crate::s_expression::*;

#[test]
pub fn equal_returns_1_when_comparing_two_same_symbols() {
    let subject = vec![
        SXRef::symbol("foo".into()),
        SXRef::symbol("foo".into()),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_symbols() {
    let subject = vec![
        SXRef::symbol("foo".into()),
        SXRef::symbol("bar".into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_one_symbol() {
    let subject = vec![
        SXRef::symbol("foo".into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_strings() {
    let subject = vec![
        SXRef::string("foo".into()),
        SXRef::string("foo".into()),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_strings() {
    let subject = vec![
        SXRef::string("foo".into()),
        SXRef::string("bar".into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_one_string() {
    let subject = vec![
        SXRef::string("foo".into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_comparing_two_nil() {
    let subject = vec![
        SXRef::nil(),
        SXRef::nil(),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_comparing_one_nil() {
    let subject = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_called_with_no_args() {
    let subject = vec![
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_numbers() {
    let subject = vec![
        SXRef::number(1),
        SXRef::number(1),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_numbers() {
    let subject = vec![
        SXRef::number(1),
        SXRef::number(2),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_one_number() {
    let subject = vec![
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_quote() {
    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_quote() {
    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        SXRef::quote(SXRef::number(2)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_one_quote() {
    let subject = vec![
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_first_or_second_arg_is_function() {
    let subject = vec![
        SXRef::function(Function::try_from("()").unwrap()),
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        SXRef::function(Function::try_from("()").unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::function(Function::try_from("()").unwrap()),
        SXRef::function(Function::try_from("()").unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_called_on_macro() {
    let subject = vec![
        SXRef::r#macro(Macro::try_from("()").unwrap()),
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        SXRef::r#macro(Macro::try_from("()").unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::r#macro(Macro::try_from("()").unwrap()),
        SXRef::r#macro(Macro::try_from("()").unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn equal_returns_nil_when_called_on_rust_macro() {
    let subject = vec![
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn equal_returns_nil_when_called_on_rust_function() {
    let subject = vec![
        RustFunction::new(dummy_fn).into(),
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
        RustFunction::new(dummy_fn).into(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        RustFunction::new(dummy_fn).into(),
        RustFunction::new(dummy_fn).into(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn equal_returns_nil_when_called_on_diff_types() {
    let subjects = vec![
        SXRef::symbol("foo".into()),
        SXRef::string("foo".into()),
        SXRef::number(1),
        SXRef::quote(SXRef::string("foo".into())),
        SXRef::r#macro("(macro () \"foo\")".try_into().unwrap()),
        SXRef::function("(lambda () \"foo\")".try_into().unwrap()),
        RustFunction::new(dummy_fn).into(),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
    ];

    let mut env = Env::new();

    for i in 0..subjects.len() {
        for j in 0..subjects.len() {
            if i == j {
                continue;
            }

            let subject = vec![
                SXRef::clone(&subjects[i]),
                SXRef::clone(&subjects[j]),
            ];

            let expected = SXRef::nil();

            let actual = McCarthyScope::equal(subject, &mut env).unwrap();

            assert_eq!(expected, actual);
        }
    }
}

#[test]
pub fn equal_ignores_args_past_2nd() {
    let subject = vec![
        SXRef::number(1),
        SXRef::number(1),
        SXRef::number(2),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn equal_returns_1_when_passed_2_samey_lists() {
    let subject = vec![
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]))
        ]),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]))
        ]),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_passed_2_different_lists() {
    let subject = vec![
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]))
        ]),
        SXRef::from(vec![
            SXRef::symbol("cdr".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]))
        ]),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::equal(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

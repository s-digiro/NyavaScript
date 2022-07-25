use super::*;

#[test]
pub fn atom_returns_1_when_called_on_nil() {
    let mut env = Env::new();

    let subject = vec![SXRef::nil()];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_symbol() {
    let mut env = Env::new();

    let subject = vec![SXRef::symbol("foo".into())];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_number() {
    let mut env = Env::new();

    let subject = vec![SXRef::number(1)];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_1_when_called_on_string() {
    let mut env = Env::new();

    let subject = vec![SXRef::string("foo".into())];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::atom(subject, &mut env);

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

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_quote() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::quote(SXRef::number(1)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_function() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::function("()".try_into().unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_macro() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::r#macro("()".try_into().unwrap()),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_rust_lambda() {
    let mut env = Env::new();

    let subject = vec![
        RustFunction::new(dummy_fn).into(),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

#[test]
pub fn atom_returns_nil_when_called_on_rust_macro() {
    let mut env = Env::new();

    let subject = vec![
        SXRef::rust_macro(RustMacro::new(dummy_macro)),
    ];

    let expected = SXRef::nil();

    let actual = McCarthyScope::atom(subject, &mut env);

    assert_eq!(expected, actual)
}

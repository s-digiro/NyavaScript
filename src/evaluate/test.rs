use super::*;

#[test]
fn eval_nil_returns_nil() {
    let subject = SXRef::nil();

    let expected = SXRef::nil();

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_number_returns_number() {
    let subject = SXRef::number(1);

    let expected = SXRef::number(1);

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_string_returns_string() {
    let subject = SXRef::string("foo".into());

    let expected = SXRef::string("foo".into());

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_undefined_symbol_returns_nil() {
    let subject = SXRef::symbol("foo".into());

    let expected = SXRef::nil();

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_defined_symbol_returns_definition() {
    let mut env = Env::new();
    env.defun("foo".into(), SXRef::number(1));

    let subject = SXRef::symbol("foo".into());

    let expected = SXRef::number(1);

    let actual = evaluate(subject, &mut env);

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_symbol_returns_symbol() {
    let subject = SXRef::quote(SXRef::symbol("foo".into()));

    let expected = SXRef::symbol("foo".into());

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_number_returns_number() {
    let subject = SXRef::quote(SXRef::number(1));

    let expected = SXRef::number(1);

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_string_returns_string() {
    let subject = SXRef::quote(SXRef::string("foo".into()));

    let expected = SXRef::string("foo".into());

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_list_returns_list() {
    let subject = SXRef::quote(SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]));

    let expected= SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = evaluate(subject, &mut Env::new());

    assert_eq!(expected, actual);
}

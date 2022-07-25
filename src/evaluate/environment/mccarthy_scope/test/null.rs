use super::*;

fn null() -> Function {
    Function::try_from(NULL).unwrap()
}

#[test]
fn null_is_valid_code() {
    null();
}

#[test]
fn null_returns_1_when_passed_nil() {
    let null = null();

    let args = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::number(1);

    let actual = null.execute(args, &mut mc_env()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn null_returns_1_when_passed_no_args() {
    let null = null();

    let args: Vec<SXRef> = vec![];

    let expected = SXRef::number(1);

    let actual = null.execute(args, &mut mc_env()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn null_returns_nil_when_passed_non_nil() {
    let null = null();

    let subjects = vec![
        vec![SXRef::from(vec![SXRef::number(1)])],
        vec![SXRef::function("()".try_into().unwrap())],
        vec![SXRef::r#macro("()".try_into().unwrap())],
        vec![SXRef::number(1)],
        vec![SXRef::quote(SXRef::number(1))],
        vec![SXRef::r#macro(RustMacro::new(dummy_macro).into())],
        vec![SXRef::string("foo".into())],
        vec![SXRef::symbol("foo".into())],
    ];

    let expected = SXRef::nil();

    for subject in subjects {
        let actual = null.execute(subject, &mut mc_env()).unwrap();

        assert_eq!(expected, actual);
    }
}

#[test]
fn null_ignores_extra_args() {
    let null = null();

    let args: Vec<SXRef> = vec![
        SXRef::nil(),
        SXRef::number(1),
    ];

    let expected = SXRef::number(1);

    let actual = null.execute(args, &mut mc_env()).unwrap();

    assert_eq!(expected, actual);

    let args: Vec<SXRef> = vec![
        SXRef::number(1),
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = null.execute(args, &mut mc_env()).unwrap();

    assert_eq!(expected, actual);
}

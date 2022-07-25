use super::*;

#[test]
pub fn pipe_with_no_args_returns_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
    ]);

    let expected =  SXRef::nil();

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn pipe_returns_result_of_only_arg_when_arg_is_atom() {
    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
        SXRef::number(1),
    ]);

    let expected = SXRef::number(1);

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
        SXRef::string("foo".into()),
    ]);

    let expected = SXRef::string("foo".into());

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
        SXRef::symbol("foo".into()),
    ]);

    let expected = SXRef::nil();

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
        SXRef::quote(SXRef::symbol("foo".into())),
    ]);

    let expected = SXRef::symbol("foo".into());

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn pipe_works() {
    let subject = SXRef::from(vec![
        SXRef::symbol("|>".into()),
        SXRef::quote(SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ])),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
        ]),
    ]);

    let expected =  SXRef::number(1);

    let actual = FunScope::pipe(subject, &mut env()).unwrap();

    assert_eq!(expected, actual)
}

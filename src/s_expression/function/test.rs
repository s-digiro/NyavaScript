use super::*;

#[test]
pub fn gets_correct_args_and_def_when_function_made_from_sxref() {
    let subject = SXRef::from(vec![
        SXRef::symbol("FAKE LAMBDA".into()),
        SXRef::from(vec![SXRef::symbol("a".into()), SXRef::symbol("b".into())]),
        SXRef::number(2),
    ]);

    let actual = Function::from(subject);

    assert_eq!(
        &vec!["a".to_owned(), "b".to_owned()],
        actual.args()
    );

    assert_eq!(SXRef::number(2), actual.definition())
}

#[test]
pub fn args_is_none_when_passed_sxref_with_non_list_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("FAKE LAMBDA".into()),
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = Function::from(subject);
    let expected: Vec<String> = Vec::new();

    assert_eq!(&expected, actual.args());

    assert_eq!(SXRef::number(2), actual.definition())
}

#[test]
pub fn args_is_none_when_passed_sxref_with_nil_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("FAKE LAMBDA".into()),
        SXRef::nil(),
        SXRef::number(2),
    ]);

    let actual = Function::from(subject);
    let expected: Vec<String> = Vec::new();

    assert_eq!(&expected, actual.args());

    assert_eq!(SXRef::number(2), actual.definition())
}

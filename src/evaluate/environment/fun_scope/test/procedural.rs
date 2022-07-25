use super::*;

#[test]
pub fn procedural_with_no_args_returns_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol(";".into()),
    ]);

    let expected =  SXRef::nil();

    let actual = FunScope::procedural(subject, &mut env()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn procedural_returns_result_of_last_evaluated_list() {
    let subject = SXRef::from(vec![
        SXRef::symbol(";".into()),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ])),
        ]),
        SXRef::from(vec![
            SXRef::symbol("cdr".into()),
            SXRef::quote(SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ])),
        ]),
    ]);

    let expected = SXRef::from(vec![SXRef::number(2)]);

    let actual = FunScope::procedural(subject, &mut env()).unwrap();

    assert_eq!(expected, actual)
}

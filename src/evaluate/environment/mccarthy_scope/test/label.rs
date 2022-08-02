use super::*;
use crate::s_expression::function::LispFunction;

#[test]
fn correctly_formed_returns_label() {
    let subject = SXRef::from(vec![
        SXRef::symbol("label".into()),
        SXRef::symbol("foo".into()),
        SXRef::from(vec![
            SXRef::symbol("lambda".into()),
            SXRef::from(vec![
                SXRef::symbol("x".into()),
            ]),
            SXRef::number(1),
        ]),
    ]);

    let expected = SXRef::from(
        LabelFunction::new(
            Some("foo".into()),
            LispFunction::new(
                vec!["x".into()],
                SXRef::number(1),
            ).into(),
        )
    );

    let actual = McCarthyScope::label(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

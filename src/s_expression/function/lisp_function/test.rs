use super::*;

#[test]
pub fn gets_correct_args_and_def_when_function_made_from_sxref() {
    let subject = SXRef::from(vec![
        SXRef::symbol("FAKE LAMBDA".into()),
        SXRef::from(vec![SXRef::symbol("a".into()), SXRef::symbol("b".into())]),
        SXRef::number(2),
    ]);

    let actual = LispFunction::from(subject);

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

    let actual = LispFunction::from(subject);
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

    let actual = LispFunction::from(subject);
    let expected: Vec<String> = Vec::new();

    assert_eq!(&expected, actual.args());

    assert_eq!(SXRef::number(2), actual.definition())
}

#[test]
pub fn from_string_works() {
    let subject = "(lambda (x) (cons x '(x ())))";

    let actual = LispFunction::try_from(subject).unwrap();

    assert_eq!(&vec!["x".to_owned()], actual.args());

    assert_eq!(
        SXRef::from(vec![
            SXRef::symbol("cons".into()),
            SXRef::symbol("x".into()),
            SXRef::from(vec![
                SXRef::symbol("quote".into()),
                SXRef::from(vec![
                    SXRef::symbol("x".into()),
                    SXRef::nil(),
                ]),
            ]),
        ]),
        actual.definition()
    )
}

#[test]
pub fn str_that_are_missing_args_work() {
    let subject = "(lambda)";

    let actual = LispFunction::try_from(subject).unwrap();
    let empty: Vec<String> = Vec::new();

    assert_eq!(&empty, actual.args());

    assert_eq!(SXRef::nil(), actual.definition())
}


#[test]
#[should_panic]
pub fn bad_lisp_fails_to_become_function() {
    let subject = "(lambda";

    LispFunction::try_from(subject).unwrap();
}

#[test]
pub fn execute_works() {
    let subject = "(lambda (x) x)";
    let subject = LispFunction::try_from(subject).unwrap();

    let args = vec![
        SXRef::number(1),
    ];

    let expected = SXRef::number(1);

    let actual = subject.execute(args, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn args_returns_list_of_args() {
    let subject = LispFunction::new(
        vec!["x".into(), "y".into()],
        SXRef::nil(),
    );

    let expected: Vec<String> = vec!["x".into(), "y".into()];

    let actual = subject.args();

    assert_eq!(&expected, actual);
}

#[test]
fn definition_returns_lambda_definition() {
    let subject = LispFunction::new(
        Vec::new(),
        SXRef::number(1),
    );

    let expected = SXRef::number(1);

    let actual = subject.definition();

    assert_eq!(expected, actual);
}

use super::*;
use crate::s_expression::function::LispFunction;
use crate::evaluate::McCarthyScope;

#[test]
fn gets_correct_label_and_function_when_made_from_sxref() {
    let subject = SXRef::from(vec![
        SXRef::symbol("Label".into()),
        SXRef::symbol("foo".into()),
        SXRef::from(vec![
            SXRef::symbol("lambda".into()),
            SXRef::from(vec![
                SXRef::symbol("a".into()),
                SXRef::symbol("b".into()),
            ]),
            SXRef::number(2),
        ]),
    ]);

    let expected = LabelFunction::new(
        Some("foo".into()),
        LispFunction::new(
            vec!["a".into(), "b".into()],
            SXRef::number(2),
            &mut Env::new(),
        ).into(),
    );

    let actual = LabelFunction::from(subject);

    assert_eq!(expected, actual);
}

#[test]
pub fn from_string() {
    let subject = "(label foo (lambda (x) (cons x '(x ()))))";

    let expected = LabelFunction::new(
        Some("foo".into()),
        LispFunction::new(
            vec!["x".into()],
            SXRef::from(vec![
                SXRef::symbol("cons".into()),
                SXRef::symbol("x".into()),
                SXRef::quote(
                    SXRef::from(vec![
                        SXRef::symbol("x".into()),
                        SXRef::nil(),
                    ]),
                ),
            ]),
            &mut Env::new(),
        ).into(),
    );

    let actual = LabelFunction::try_from(subject).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn str_that_is_missing_args_work() {
    let subject = "(label)";

    let expected = LabelFunction::new(
        None,
        LispFunction::new(
            Vec::new(),
            SXRef::nil(),
            &mut Env::new(),
        ).into(),
    );

    let actual = LabelFunction::try_from(subject).unwrap();

    assert_eq!(expected, actual);
}


#[test]
#[should_panic]
pub fn bad_lisp_fails_to_become_function() {
    let subject = "(label";

    LispFunction::try_from(subject).unwrap();
}

#[test]
pub fn execute_works() {
    let subject = "
        (label first-atom
            (lambda (x)
                (cond
                    ((atom x) x)
                    (1 (first-atom (car x))))))
    ";

    let subject = LabelFunction::try_from(subject).unwrap();

    let args = vec![
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
            ]),
            SXRef::number(3),
        ]),
    ];

    let expected = SXRef::number(1);

    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let actual = subject.execute(args, &mut env).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn label_returns_label() {
    let subject = LabelFunction::new(
        Some("foo".into()),
        SXRef::nil(),
    );

    let expected = Some("foo");

    let actual = subject.label();

    assert_eq!(expected, actual);
}

#[test]
fn function_returns_function() {
    let subject = LabelFunction::new(
        Some("foo".into()),
        SXRef::number(1),
    );

    let expected = SXRef::number(1);

    let actual = subject.function();

    assert_eq!(&expected, actual);
}

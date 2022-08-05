use super::*;

#[test]
fn list_with_no_args_procudes_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("list".into()),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::list(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn one_arg() {
    let subject = SXRef::from(vec![
        SXRef::symbol("list".into()),
        SXRef::number(1),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
    ]);

    let actual = McCarthyScope::list(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn two_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("list".into()),
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
    ]);

    let actual = McCarthyScope::list(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn five_args() {
    let subject = SXRef::from(vec![
        SXRef::symbol("list".into()),
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
        SXRef::number(4),
        SXRef::number(5),
    ]);

    let expected = SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
        SXRef::number(3),
        SXRef::number(4),
        SXRef::number(5),
    ]);

    let actual = McCarthyScope::list(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}


use super::*;

fn and() -> Function {
    Function::try_from(AND).unwrap()
}

#[test]
fn and_is_valid_code() {
    and();
}

#[test]
fn and_returns_nil_when_passed_no_args() {
    let and = and();

    let args: Vec<SXRef> = vec![];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn and_returns_nil_when_passed_1_args() {
    let and = and();

    let args = vec![
        SXRef::number(2),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn and_returns_second_arg_when_passed_2_non_nil_args() {
    let and = and();

    let args = vec![
        SXRef::number(2),
        SXRef::string("foo".into()),
    ];

    let expected = SXRef::string("foo".into());

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::string("foo".into()),
        SXRef::number(2),
    ];

    let expected = SXRef::number(2);

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn and_returns_nil_when_passed_1_non_nil_and_1_nil_args() {
    let and = and();

    let args = vec![
        SXRef::nil(),
        SXRef::number(2),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::number(2),
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn and_returns_nil_when_passed_2_nil_args() {
    let and = and();

    let args = vec![
        SXRef::nil(),
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn and_ignores_extra_args() {
    let and = and();

    let args = vec![
        SXRef::nil(),
        SXRef::nil(),
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::nil(),
        SXRef::number(1),
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::number(1),
        SXRef::number(1),
        SXRef::nil(),
    ];

    let expected = SXRef::number(1);

    let actual = and.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

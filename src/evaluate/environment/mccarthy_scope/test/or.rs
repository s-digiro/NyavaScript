use super::*;

fn or() -> Function {
    Function::try_from(OR).unwrap()
}

#[test]
fn or_is_valid_code() {
    or();
}

#[test]
fn or_returns_nil_when_passed_no_args() {
    let or = or();

    let args: Vec<SXRef> = vec![];

    let expected = SXRef::nil();

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_returns_nil_when_passed_1_nil_args() {
    let or = or();

    let args = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_returns_arg_when_passed_non_nil_arg() {
    let or = or();

    let args = vec![
        SXRef::number(2),
    ];

    let expected = SXRef::number(2);

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_returns_first_arg_when_passed_2_non_nil_args() {
    let or = or();

    let args = vec![
        SXRef::number(2),
        SXRef::string("foo".into()),
    ];

    let expected = SXRef::number(2);

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::string("foo".into()),
        SXRef::number(2),
    ];

    let expected = SXRef::string("foo".into());

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_returns_first_arg_when_passed_1_non_nil_arg_and_1_nil() {
    let or = or();

    let args = vec![
        SXRef::number(2),
        SXRef::nil(),
    ];

    let expected = SXRef::number(2);

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_returns_2nd_arg_when_passed_1_nil_arg_and_1_non_nil() {
    let or = or();

    let args = vec![
        SXRef::nil(),
        SXRef::string("foo".into()),
    ];

    let expected = SXRef::string("foo".into());

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn or_ignores_extra_args() {
    let or = or();

    let args = vec![
        SXRef::nil(),
        SXRef::nil(),
        SXRef::number(1),
    ];

    let expected = SXRef::nil();

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::nil(),
        SXRef::number(1),
        SXRef::number(2),
    ];

    let expected = SXRef::number(1);

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::number(2),
        SXRef::number(1),
        SXRef::nil(),
    ];

    let expected = SXRef::number(2);

    let actual = or.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

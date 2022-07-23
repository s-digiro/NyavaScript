use super::*;

fn not() -> Function {
    Function::try_from(NOT).unwrap()
}

#[test]
fn not_is_valid_code() {
    not();
}

#[test]
fn not_returns_1_when_passed_no_args() {
    let not = not();

    let args: Vec<SXRef> = vec![];

    let expected = SXRef::number(1);

    let actual = not.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn not_returns_1_when_passed_nil_arg() {
    let not = not();

    let args = vec![
        SXRef::nil(),
    ];

    let expected = SXRef::number(1);

    let actual = not.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn not_returns_nil_when_passed_non_nil_arg() {
    let not = not();

    let args = vec![
        SXRef::number(2),
    ];

    let expected = SXRef::nil();

    let actual = not.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

#[test]
fn not_ignnotes_extra_args() {
    let not = not();

    let args = vec![
        SXRef::nil(),
        SXRef::number(2),
    ];

    let expected = SXRef::number(1);

    let actual = not.execute(args, &mut mc_env());

    assert_eq!(expected, actual);

    let args = vec![
        SXRef::number(2),
        SXRef::nil(),
    ];

    let expected = SXRef::nil();

    let actual = not.execute(args, &mut mc_env());

    assert_eq!(expected, actual);
}

use super::*;

#[test]
fn get_symbol_one() {
    let subject = test_lib();

    let actual = subject.get_sym("one");

    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    let actual = f.call(Vec::new());
    let expected = 1;

    assert_eq!(expected, actual);
}

#[test]
fn get_symbol_two() {
    let subject = test_lib();

    let actual = subject.get_sym("two");

    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    let actual = f.call(Vec::new());
    let expected = 2;

    assert_eq!(expected, actual);
}

#[test]
fn get_symbol_same() {
    let subject = test_lib();

    let actual = subject.get_sym("same");

    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    let actual = f.call(vec![5]);
    let expected = 5;

    assert_eq!(expected, actual);
}

#[test]
fn get_symbol_add() {
    let subject = test_lib();
    let actual = subject.get_sym("add");
    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    let actual = f.call(vec![2, 1]);
    let expected = 3;

    assert_eq!(expected, actual);
}

#[test]
fn get_symbol_func() {
    let subject = test_lib();
    let actual = subject.get_sym("func");
    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    let actual = f.call(vec![1, 2]);
    let expected = 1;

    assert_eq!(expected, actual);
}


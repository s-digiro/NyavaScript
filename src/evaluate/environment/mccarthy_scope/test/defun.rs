use super::*;

#[test]
pub fn defun_adds_well_formed_function_to_global_scope() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("defun".into()),
        SXRef::symbol("foo".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::symbol("x".into()),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::defun(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let f = env.get("foo");

    let f = if let SX::Function(f) = &*f {
        f
    } else {
        panic!("Expected f to be a SX::Function. Instead it was {:?}", f)
    };

    let f = if let Function::Lisp(l) = f {
        l
    } else {
        panic!("Expected f to be a Function::Lisp. Instead it was {:?}", f)
    };

    assert_eq!(
        f.args(),
        &vec!["x".to_owned()],
    );

    assert_eq!(
        f.definition(),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::symbol("x".into()),
        ]),
    );
}

#[test]
pub fn defun_adds_function_to_global_scope_with_nil_def_when_missing_def() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("defun".into()),
        SXRef::symbol("foo".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::defun(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let f = env.get("foo");

    let f = if let SX::Function(f) = &*f {
        f
    } else {
        panic!("Expected f to be a SX::Function. Instead it was {:?}", f)
    };

    let f = if let Function::Lisp(l) = f {
        l
    } else {
        panic!("Expected f to be a Function::Lisp. Instead it was {:?}", f)
    };

    assert_eq!(
        f.args(),
        &vec!["x".to_owned()],
    );

    assert_eq!(
        f.definition(),
        SXRef::nil(),
    );
}

#[test]
pub fn defun_adds_function_with_nil_def_and_args_to_global_scope_when_missing_def_and_args() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("defun".into()),
        SXRef::symbol("foo".into()),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::defun(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let f = env.get("foo");

    let f = if let SX::Function(f) = &*f {
        f
    } else {
        panic!("Expected f to be a SX::Function. Instead it was {:?}", f)
    };

    let f = if let Function::Lisp(l) = f {
        l
    } else {
        panic!("Expected f to be a Function::Lisp. Instead it was {:?}", f)
    };

    let empty: Vec<String> = Vec::new();

    assert_eq!(
        f.args(),
        &empty,
    );

    assert_eq!(
        f.definition(),
        SXRef::nil(),
    );
}

#[test]
pub fn defun_does_nothing_when_missing_function_name() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("defun".into()),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::defun(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let f = env.get("foo");

    match &*f {
        SX::Nil => (),
        _ => panic!("Expected f to be a SX::Nil. Instead it was {:?}", f),
    }
}

#[test]
pub fn defun_can_define_lambda_with_no_args() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("defun".into()),
        SXRef::symbol("foo".into()),
        SXRef::from(vec![]),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::symbol("x".into()),
        ]),
    ]);

    let expected = SXRef::nil();

    let actual = McCarthyScope::defun(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let f = env.get("foo");

    let f = if let SX::Function(f) = &*f {
        f
    } else {
        panic!("Expected f to be a SX::Function. Instead it was {:?}", f)
    };

    let f = if let Function::Lisp(l) = f {
        l
    } else {
        panic!("Expected f to be a Function::Lisp. Instead it was {:?}", f)
    };

    let empty: Vec<String> = Vec::new();

    assert_eq!(
        f.args(),
        &empty,
    );

    assert_eq!(
        f.definition(),
        SXRef::from(vec![
            SXRef::symbol("car".into()),
            SXRef::symbol("x".into()),
        ]),
    );
}
